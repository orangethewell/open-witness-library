import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';


const PubCatalog = ({ publications }) => {
    const [loadedIdx, setLoadedIdx] = useState(0);  // `loaded_idx`
    const limit = 10;  // Defina o limite conforme necessário
    const navigate = useNavigate();

    console.log(publications)

    const handleClick = async (publication) => {
        let pubSymbol = `${publication.symbol}_${publication.language}`;  // TODO: Remover este sufixo
        await invoke("pubcatalog_set_media_location", {lang: 'T', category: 'bk', pubSymbol})
        navigate(`/summary/${publication.language}/${publication.category}/${pubSymbol}`);
    };

    return (
        <ul>
            {publications.map((publication) => (
                <li key={publication.symbol}>
                    <a onClick={() => handleClick(publication)}>
                        <div>
                            <img src={convertFileSrc(publication.cover_icon_path)} alt={publication.title} />
                            <p>{publication.title}</p>
                        </div>
                    </a>
                </li>
            ))}
        </ul>
    );
};

export default PubCatalog;
