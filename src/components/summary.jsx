import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

const PubSummary = ({ chapters, lang, category, pubSymbol }) => {
    const navigate = useNavigate();

    const handleChapterClick = (chapter) => {
        navigate(`/pubview/${lang}/${category}/${pubSymbol}/${chapter.id}`);
    };

    return (
        <div>
            {chapters.length > 0 ? (
                <ul>
                    {chapters.map((chapter) => (
                        <li key={chapter.id}>
                            <a onClick={() => handleChapterClick(chapter)}>
                                <p>{chapter.title}</p>
                            </a>
                        </li>
                    ))}
                </ul>
            ) : (
                <div>
                    <p>{"Please, Wait..."}</p>
                </div>
            )}
        </div>
    );
};

export default PubSummary;
