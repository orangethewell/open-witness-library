import { Box, List, ListItemButton, ListItemText, Tab, Tabs } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

const DocumentItemButton = (props) => {
    let {item, finder, symbol} = props;
    const navigate = useNavigate();

    return (<ListItemButton sx={{
        height: 80
    }} key={item.id} onClick={(ev) => {
        navigate(`/publication/${symbol}/${finder(item.id).document.id}`);
    }}>
        <ListItemText primaryTypographyProps={{
            variant: "body2",
            color: "textSecondary"
        }} secondaryTypographyProps={{
            variant: "body1",
            color: "textPrimary"
        }} secondary={finder(item.id).document.toc_title ? (finder(item.id).document.toc_title) : (item.title)} primary={finder(item.id).document.context_title ? (finder(item.id).document.context_title) : undefined}/>
    </ListItemButton>)
}

const SectionViewPanel = (props) => {
    const { children, value, index, ...other} = props;
    return (
    <div
        role="tabpanel"
        hidden={value !== index}
        id={`tabpanel-${index}`}
        aria-labelledby={`tab-${index}`}
        {...other}
    >
        {value === index && <Box>{children}</Box>}
    </div>
    )
}

const PublicationView = () => {
    const { symbol } = useParams();

    const [viewItems, setViewItems] = useState({
        publication_view_items: [],
        publication_view_items_documents: [],
    });
    
    const [tabIndex, setTabIndex] = useState(0);

    const handleTabSwitch = (event, newValue) => {
        setTabIndex(newValue);
      };

    useEffect(() => {
        const fetchViewItems = async () => {
            const viewItem = await invoke("catalog_get_publication_view_from", {filenameSymbol: symbol});
            const documents = await fetchDocuments(viewItem.publication_view_items_documents);
            setViewItems({
                publication_view_items_documents: documents, 
                publication_view_items: buildHierarchy(viewItem.publication_view_items)
            });
        };
        fetchViewItems();
    }, []);

    const fetchDocuments = async (items) => {
        let documents = [];
        for (const item of items) {
            const document = await invoke("catalog_get_document_by_id", {
                documentId: item.document_id
            });
            documents.push({ ...item, document });
        }
        return documents;
    }

    const buildHierarchy = (items) => {
        const itemMap = new Map();
      
        // Cria um mapa para acesso rápido dos itens pelo ID
        items.forEach((item) => {
          itemMap.set(item.id, { ...item, children: [] });
        });
      
        const hierarchy = [];
      
        // Monta a hierarquia pai-filho
        items.forEach((item) => {
          if (item.parent_publication_view_item_id === -1) {
            // Adiciona diretamente ao nível superior
            hierarchy.push(itemMap.get(item.id));
          } else {
            // Adiciona como filho do item pai
            const parent = itemMap.get(item.parent_publication_view_item_id);
            if (parent) {
              parent.children.push(itemMap.get(item.id));
            }
          }
        });
      
        return hierarchy;
      };

    function findDocumentByViewItemId(id) {
        let item = viewItems.publication_view_items_documents.filter((element) => {
            return element.publication_view_item_id === id;
        })[0];
        return item;
    }

    return (
        <Box sx={{ width: '100%' }}>
            <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
                {viewItems.publication_view_items.length > 1 ? (
                    <Tabs value={tabIndex} onChange={handleTabSwitch} aria-label="Publication Sections">
                        {viewItems.publication_view_items.map((item, index) => (
                            <Tab label={item.title} key={index} />
                        ))}
                    </Tabs>
                ) : undefined}
            </Box>
            <Box>
                {viewItems.publication_view_items.map((section, index) => (
                    <SectionViewPanel key={index} value={tabIndex} index={index}>
                        <List>
                        {section.children.map((item) => (
                            <>
                                {item.default_document_id!== -1? (
                                    <>
                                    { findDocumentByViewItemId(item.id).document.toc_title ? (
                                        <DocumentItemButton symbol={symbol} item={item} finder={findDocumentByViewItemId}/>
                                    ) : undefined}
                                    </>
                                ): (
                                    <Box key={item.id} sx={{ fontWeight: 500, fontSize: 18, paddingTop: 4 }}>{item.title}</Box>
                                )}
                                {item.children.map((subitem) => (
                                    <>
                                    {subitem.default_document_id!== -1? (
                                        <>
                                        { findDocumentByViewItemId(subitem.id).document.toc_title ? (
                                            <DocumentItemButton symbol={symbol} item={subitem} finder={findDocumentByViewItemId}/>
                                        ) : undefined}
                                        </>
                                    ): (
                                        <Box key={subitem.id} sx={{ fontWeight: 500, fontSize: 16 }}>{subitem.title}</Box>
                                    )}
                                    </>
                                ))}
                            </>
                        ))}
                        </List>
                    </SectionViewPanel>
                ))}
            </Box>
        </Box>
    );
};

export default PublicationView;