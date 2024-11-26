import { Box, Button } from "@mui/material";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { GrNext, GrPrevious } from "react-icons/gr";
import { Swiper, SwiperSlide, useSwiper } from "swiper/react";
import "swiper/css"
import { Navigation, Virtual } from "swiper/modules";
import { platform } from "@tauri-apps/plugin-os";

const PrevDocumentButton = () => {
    const swiper = useSwiper();

    return (
        <button style={{
            display: platform() === "android" || platform() === "ios" ? "none" : "block",
        }} className='go-route-pub go-pub-back' onClick={() => swiper.slidePrev()}>
            <GrPrevious />
        </button>
    )
}

const NextDocumentButton = () => {
    const swiper = useSwiper();

    return (
        <button style={{
            display: platform() === "android" || platform() === "ios" ? "none" : "block",
        }} className='go-route-pub go-pub-next' onClick={() => swiper.slideNext()}>
            <GrNext />
        </button>
    )
}

const useSelectionText = (ref) => {
    const [data, setData] = useState({showTools: false});

    function tokenizeSelection(selection) {
        let tokenId = 0;
    
        function tokenizeNode(node, rangeStart, rangeEnd) {
            const text = node.textContent;
    
            // Define os limites do texto que será processado
            const start = rangeStart !== null ? rangeStart : 0;
            const end = rangeEnd !== null ? rangeEnd : text.length;
    
            const tokens = text.slice(start, end).match(/[\w\u00C0-\u017F]+|[^\w\s]/g) || [];
            let result = [];
            let offset = start;
    
            tokens.forEach(token => {
                const tokenStart = text.indexOf(token, offset);
                const tokenEnd = tokenStart + token.length;
    
                let range = document.createRange();
                range.setStart(node, tokenStart);
                range.setEnd(node, tokenEnd);
    
                result.push({
                    type: "text",
                    idx: tokenId++,
                    content: token,
                    parent: node.parentNode,
                    range: range.cloneRange()
                });
    
                offset = tokenEnd;
            });
    
            return result;
        }
    
        function processNode(node, range) {
            let result = [];
    
            if (node.nodeType === Node.TEXT_NODE) {
                const startOffset = node === range.startContainer ? range.startOffset : null;
                const endOffset = node === range.endContainer ? range.endOffset : null;
    
                result.push(...tokenizeNode(node, startOffset, endOffset));
            } else if (node.nodeType === Node.ELEMENT_NODE) {
                node.childNodes.forEach(child => {
                    const childRange = range.cloneRange();
                    if (childRange.intersectsNode(child)) {
                        result.push(...processNode(child, childRange));
                    }
                });
            }
    
            return result;
        }
    
        // Verifica se há uma seleção válida
        if (!selection || selection.rangeCount === 0) {
            return [];
        }
    
        const range = selection.getRangeAt(0);
        return processNode(range.commonAncestorContainer, range);
    }    

    function tokenizeParagraph(rootElement) {
        let pTokenId = 0;
        const tokens = [];
    
        function tokenizeNode(node) {
            const text = node.textContent;
            const parent = node.parentNode;
            const matchedTokens = text.match(/[\w\u00C0-\u017F]+|[^\w\s]/g) || [];
            let offset = 0;
    
            matchedTokens.forEach(token => {
                const tokenStart = text.indexOf(token, offset);
                const tokenEnd = tokenStart + token.length;
    
                const range = document.createRange();
                range.setStart(node, tokenStart);
                range.setEnd(node, tokenEnd);
    
                tokens.push({
                    type: "text", // Indica que é um token de texto
                    idx: pTokenId++, // ID único do token
                    content: token, // O conteúdo do token
                    parent: parent, // Referência ao elemento pai no DOM
                    childNodeIndex:  Array.from(parent.childNodes).indexOf(node),
                    range: range, // Range associado ao token
                });
    
                offset = tokenEnd;
            });
        }
    
        function processNode(node) {
            if (node.nodeType === Node.ELEMENT_NODE) {
                node.childNodes.forEach(processNode);
            } else if (node.nodeType === Node.TEXT_NODE) {
                tokenizeNode(node);
            }
        }
    
        processNode(rootElement);
    
        return tokens;
    }

    function highlightTokens(tokens, selected) {
        function getSelectionIndices(paragraphTokens, selectionTokens) {
            // Obter o índice do primeiro token selecionado no parágrafo
            const startIdx = paragraphTokens.findIndex(
                (token) => token.content.endsWith(selectionTokens[0].content) && token.range.endOffset === selectionTokens[0].range.endOffset
            );
        
            // Obter o índice do último token selecionado no parágrafo
            const endIdx = paragraphTokens.findIndex(
                (token) => token.content.startsWith(selectionTokens[selectionTokens.length - 1].content) && token.range.startOffset === selectionTokens[selectionTokens.length - 1].range.startOffset
            );
        
            return { startIdx, endIdx };
        }

        function highlightTokens(tokens, startIdx, endIdx) {
            const startToken = tokens[startIdx];
            const endToken = tokens[endIdx];

            const getInnerNodesCount = () => {
                let count = 0;
                let parent = startToken.parent;
                
                let end = endIdx;

                for (var idx = startIdx; idx <= end; idx++) {
                    if (tokens[idx].parent != parent) {
                        count++;
                        parent = tokens[idx].parent;
                    }
                }

                return count;
            }

            console.log(getInnerNodesCount());

            if (getInnerNodesCount() == 0) {
                const parent = startToken.parent;
                const range = document.createRange();
                range.setStart(parent.childNodes[startToken.childNodeIndex], startToken.range.startOffset);
                range.setEnd(parent.childNodes[startToken.childNodeIndex], endToken.range.endOffset);
                let span = document.createElement("span");
                span.style.backgroundColor = "#ffff00a5";
                range.surroundContents(span);
                console.log("Deu certo")
            } else {
                let startRange = startToken.range.startOffset;
                let startRangeParent = startToken.parent;
                
                for (var idx = startIdx; idx <= endIdx; idx++) {
                    if (tokens[idx].parent != startRangeParent) {
                        let textChildNode = tokens[idx - 1].parent.childNodes[tokens[idx - 1].childNodeIndex];
                        let endRange = tokens[idx - 1].range.endOffset;
                        console.log("marking: ", startRange, endRange, " at ", textChildNode);
                        const range = document.createRange();
                        range.setStart(textChildNode, startRange);
                        range.setEnd(textChildNode, endRange);
                        let span = document.createElement("span");
                        span.style.backgroundColor = "#ffff00a5";
                        range.surroundContents(span);
                        startRange = tokens[idx].range.startOffset;
                        startRangeParent = tokens[idx].parent;
                    }
                }
            }
        }

        let {startIdx, endIdx} = getSelectionIndices(tokens, selected);
        highlightTokens(tokens, startIdx, endIdx);
    }

    function getSelectedTokens(rootElement, tokens) {
        const selection = document.getSelection();
    
        if (selection.rangeCount == 0) {
            return;
        }

        const { x, y, width } = selection.getRangeAt(0).getBoundingClientRect();

        const selectionTokens = tokenizeSelection(selection);
        if (selectionTokens.length > 0) {
            setData({
                showTools: true,
                x: x,
                y: y,
                width,
                element: (
                    <Box
                        sx={{
                            position: "absolute",
                            left: `${data.x + data.width / 2}px`,
                            top: `${data.y}px`,
                            display: "inline-block",
                            textAlign: "center",
                        }}
                    >
                        <Button 
                            sx={{
                                backgroundColor: "#eeee00",
                                border: "#555500 solid 1px",
                                borderRadius: "100%",
                                minWidth: 16,
                                minHeight: 16,
                                padding: 1.5
                            }}
                            onClick={() => highlightTokens(tokens, selectionTokens)}
                        ></Button>
                    </Box>
                )
            })
        } else {
            setData({showTools: false});
        }
        console.log(selectionTokens)
        console.log(tokens);
    }

    const onMouseup = () => {
        const startNode = document.getSelection().getRangeAt(0).startContainer.parentElement.closest('[data-pid]');
        const tokens = tokenizeParagraph(startNode);
    
        const selectedTokenIds = getSelectedTokens(startNode, tokens);
        console.log(selectedTokenIds); // Lista de índices dos tokens selecionados
    };

    useEffect(() => {
        document.addEventListener("mouseup", onMouseup);

        return () => {
            document.removeEventListener("mouseup", onMouseup);
        }
    }, []);

    return data;
}

const Document = (params) => {
    const { documentId, currentId } = params;
    const [content, setContent] = useState("");
    const contentRef = useRef(null);
    const swiper = useSwiper();

    const data = useSelectionText(contentRef);

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
            {data.showTools && (
                data.element
        )}
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
                simulateTouch={false}
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