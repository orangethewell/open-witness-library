import { Box, List, ListItemButton, ListItemText, Tab, Tabs } from "@mui/material";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { GrNext, GrPrevious } from "react-icons/gr";
import { Swiper, SwiperSlide, useSwiper } from "swiper/react";
import "swiper/css"
import { Navigation, Virtual } from "swiper/modules";

const PrevDocumentButton = () => {
    const swiper = useSwiper();

    return (
        <button className='go-route-pub go-pub-back' onClick={() => swiper.slidePrev()}>
            <GrPrevious />
        </button>
    )
}

const NextDocumentButton = () => {
    const swiper = useSwiper();

    return (
        <button className='go-route-pub go-pub-next' onClick={() => swiper.slideNext()}>
            <GrNext />
        </button>
    )
}

const Document = (params) => {
    const { documentId, currentId } = params;
    const [content, setContent] = useState("");
    const contentRef = useRef(null);
    const swiper = useSwiper();

    swiper.slideTo(currentId);

    useEffect(() => {
        const fetchData = async () => {
            setContent(await invoke("catalog_get_document_content", {documentId: documentId}));
        };
        fetchData();
    }, [documentId]);

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
                    const newSrc = convertFileSrc(imageName, 'jwpub-media');
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
                    const newSrc = convertFileSrc(imgPath, 'jwpub-media');
        
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
        <div
            style={{
                height: "calc(100vh - 48px)",
                overflow: "auto",
            }}
        >
        <article 
            style={{
                paddingTop: 8,
                paddingBottom: 12,
            }} 
            id='article' 
            className='
                jwac 
                docClass-13 
                docId-1102023301 
                ms-ROMAN 
                ml-T 
                dir-ltr 
                pub-lmd 
                layout-reading 
                layout-sidebar' 
            ref={contentRef} 
        />
        </div>
    )
}

const DocumentView = () => {
    const { symbol, documentId } = useParams();
    const [id, setId] = useState(0);
    const [documents, setDocuments] = useState([]);

    const navigate = useNavigate();

    useEffect(() => {
        const fetchData = async () => {
            await invoke("catalog_open_connection", {filenameSymbol: symbol});
            setDocuments(await invoke("catalog_get_documents"));
        };
        fetchData();
        setId(parseInt(documentId));
    }, [symbol]);

    const handleSlideChange = (swiper) => {
        const newIndex = swiper.activeIndex;
        const newDocId = documents[newIndex]?.id;
        setId(newDocId);

        if (newDocId) {
            navigate(`/publication/${symbol}/${newDocId}`); // Atualiza a rota
        }
    };

    return (
        <Box>
            <Swiper
                modules={[Virtual, Navigation]}
                virtual
                navigation={{
                    nextEl: '.go-route-pub .go-pub-next',
                    prevEl: 'go-route-pub go-pub-back',
                }}
                slides={documents}
                slidesPerView={1}
                initialSlide={id}
                onSlideChange={handleSlideChange}
            >
            <PrevDocumentButton/>
            {
                documents.map((document, index) => {
                    return (
                        <SwiperSlide key={index} virtualIndex={document.id}>
                            <Document documentId={document.id} currentId={id}/>
                        </SwiperSlide>
                    );
                })
            }
            <NextDocumentButton/>
            </Swiper>
        </Box>
    );
};

export default DocumentView;