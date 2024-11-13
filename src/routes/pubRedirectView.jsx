import React from 'react';
import { redirect, useParams } from 'react-router-dom'; // Usando React Router para redirecionamento

const PubViewRedirect = () => {
    const { lang, category, pubSymbol, chapterId } = useParams(); // Usando React Router para recuperar os parâmetros da URL
    redirect(`/pubview/${lang}/${category}/${pubSymbol}/${chapterId}`)
    return (
        <>
        </>
    );
};

export default PubViewRedirect;
