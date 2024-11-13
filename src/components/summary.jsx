import { List, ListItemButton, ListItemText } from '@mui/material';
import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

const PubSummary = ({ chapters, lang, category, pubSymbol }) => {
    const navigate = useNavigate();

    const handleChapterClick = (chapter) => {
        navigate(`/pubview/${lang}/${category}/${pubSymbol}/${chapter.id}`);
    };

    console.log(chapters)

    return (
        <div>
            {chapters.length > 0 ? (
                <List>
                    {chapters.map((chapter) => (
                        <ListItemButton key={chapter.id} onClick={() => handleChapterClick(chapter)}>
                            <ListItemText>{(
                                <p>{chapter.context_title ? (
                                    <>
                                    <span className='subtext'>{chapter.context_title}</span>
                                    <br/>
                                    </>
                                ): undefined}{chapter.title}</p>
                            )}
                            </ListItemText>
                        </ListItemButton>
                    ))}
                </List>
            ) : (
                <div>
                    <p>{"Please, Wait..."}</p>
                </div>
            )}
        </div>
    );
};

export default PubSummary;
