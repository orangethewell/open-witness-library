import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import React, { useState, useEffect, useMemo, useRef } from 'react';
import { useNavigate, useParams } from 'react-router-dom';

const ChapterView = () => {
    const { lang, category, pubSymbol, chapterId } = useParams();
    let chapterIdx = Number(chapterId);
    const [content, setContent] = useState({ content: '', next_exists: false, previous_exists: false });
    const navigate = useNavigate();
    const contentRef = useRef(null);

    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = '/pub-styling.css'
    document.head.appendChild(link);

    useEffect(() => {
        const fetchData = async () => {
            setContent(await invoke("pubcatalog_get_chapter_content", {lang, category, pubSymbol, contentId: chapterIdx}));
        };
        fetchData();
    }, [lang, category, pubSymbol, chapterId]);

    const prevChapter = content.previous_exists ? (
        <button className='go-route-pub go-pub-back' onClick={() => navigate(`/pubview/${lang}/${category}/${pubSymbol}/${chapterIdx - 1}`)}>
            {"<"}
        </button>
    ) : null;

    const nextChapter = content.next_exists ? (
        <button className='go-route-pub go-pub-next' onClick={() => navigate(`/pubview/${lang}/${category}/${pubSymbol}/${chapterIdx + 1}`)}>
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
            if (content.content && contentRef.current) {
                let updatedContent = await updateImageSources(content.content);
                updatedContent = await updateBackgroundImgSources(updatedContent);
                contentRef.current.innerHTML = updatedContent;
            }
        };

        updateContent();
    }, [content]);

    return (
        <div>
            {prevChapter}
            <article id='article' className='jwac docClass-13 docId-1102023301 ms-ROMAN ml-T dir-ltr pub-lmd layout-reading layout-sidebar' ref={contentRef} />
            {nextChapter}
        </div>
    );
};

export default ChapterView;
