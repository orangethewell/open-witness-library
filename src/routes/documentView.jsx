import { Box, List, ListItemButton, ListItemText, Tab, Tabs } from "@mui/material";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

const DocumentView = () => {
    const { symbol, documentId } = useParams();
    const Id = parseInt(documentId)

    const [content, setContent] = useState("");
    const [prevExists, setPrevExists] = useState(false);
    const [nextExists, setNextExists] = useState(false);

    const contentRef = useRef(null);

    const navigate = useNavigate();

    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = '/pub-styling.css'
    document.head.appendChild(link);
    useEffect(() => {
        const fetchData = async () => {
            await invoke("catalog_open_connection", {filenameSymbol: symbol});
            setContent(await invoke("catalog_get_document_content", {documentId: Id}));
            setPrevExists(await invoke("catalog_check_document_exists", {documentId: Id - 1}));
            setNextExists(await invoke("catalog_check_document_exists", {documentId: Id + 1}));

        };
        fetchData();
    }, [symbol, documentId]);

    const prevDocument = prevExists ? (
        <button className='go-route-pub go-pub-back' onClick={() => navigate(`/publication/${symbol}/${Id - 1}`)}>
            {"<"}
        </button>
    ) : null;

    const nextDocument = nextExists ? (
        <button className='go-route-pub go-pub-next' onClick={() => navigate(`/publication/${symbol}/${Id + 1}`)}>
            {">"}
        </button>
    ) : null;

    useEffect(() => {
        const updateImageSources = async (contentHtml) => {
            const parser = new DOMParser();
            let doc = parser.parseFromString(contentHtml, 'text/html');
            const images = doc.querySelectorAll('img');

            for (let img of images) {
                const src = img.getAttribute('src');
                if (src && src.startsWith('jwpub-media://')) {
                    const imageName = src.split('jwpub-media://')[1];
                    // Convertendo o caminho do arquivo usando convertFileSrc
                    const newSrc = await convertFileSrc(imageName, 'jwpub-media');
                    img.setAttribute('src', newSrc);  // Atualiza o src da imagem
                }
            }

            // Retorna o HTML atualizado
            return doc.body.innerHTML;
        };

        const updateBackgroundImgSources = async (contentHtml) => {
            const parser = new DOMParser();
            let doc = parser.parseFromString(contentHtml, 'text/html');
            const elements = doc.querySelectorAll('[data-bg-image]');

            for (const element of elements) {
                const imgPath = element.getAttribute('data-bg-image').replace('jwpub-media://', '');
                
                try {
                    // Converte o caminho usando `convertFileSrc`
                    const newSrc = await convertFileSrc(imgPath, 'jwpub-media');
        
                    // Define o estilo `background-image` com a URL convertida
                    element.style.backgroundImage = `url('${newSrc}')`;
                } catch (error) {
                    console.error(`Erro ao converter o caminho da imagem para o elemento: ${element}`, error);
                }
            }

            return doc.body.innerHTML;
        }

        const updateContent = async () => {
            if (content && contentRef.current) {
                let updatedContent = await updateImageSources(content);
                updatedContent = await updateBackgroundImgSources(updatedContent);
                contentRef.current.innerHTML = updatedContent;
            }
        };

        updateContent();
    }, [content]);

    return (
        <Box>
            {prevDocument}
            <article id='article' className='jwac docClass-13 docId-1102023301 ms-ROMAN ml-T dir-ltr pub-lmd layout-reading layout-sidebar' ref={contentRef} />
            {nextDocument}
        </Box>
    );
};

export default DocumentView;