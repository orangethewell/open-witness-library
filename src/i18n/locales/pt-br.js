export default {
    translations: {
        home: {
            welcome: 'Bem-vindo ao <i>Open Witness Library</i>',
            description: 'Um programa para ler e organizar publicações que carregam o <strong>verdadeiro nome de Deus</strong>.',
            disclaimer: 'Esse programa não baixa publicações ainda, acesse o site para baixar publicações e clique no <plus_icon /> para instalar a publicação.',
            download_button: 'Baixar publicações',
            local_button: 'Acessar biblioteca local'
        },
        menu: {
            home: 'Início',
            library: 'Biblioteca',
            settings: 'Configurações',
        },
        library_categories: {
            publication_types: {
                booklets: "Livretos"
            }
        },
        library: {
            title: "Biblioteca",
            publication_types: {
                bible: "Bíblias",
                tract: "Convites e tratados",
                index: "Índice",
                kingdom_ministry: "Ministério do Reino",
                book: "Livros",
                brochure: "Brochuras e livretos",
                watchtower: "A Sentinela",
                awake: "Despertai!",
                meeting_workbook: "Apostilas",
                program: "Programas",
                web: "Série de Artigos",
                manual_guidelines: "Orientações",
            }
        },
        settings: {
            alerts: {
                missing_assets: "Arquivos de estilo faltando, por favor, clique no botão para baixar.",
                missing_assets_button1: "Baixar",
                failed_download: "O download falhou.",
                failed_download_button1: "Tentar novamente",
                download_progress: "Baixando...",
                download_finished: "Download concluído! Recarregando o programa em 5 segundos..."
            },
            title: 'Configurações',
            display: 'Exibição',
            language: 'Idioma',
            language_selectors: {
                "en-US": 'Inglês',
                "pt-BR": 'Português (Brasil)'
            },
            appearance: 'Aparência',
            appearance_selectors: {
                default: 'Padrão do sistema',
                light: 'Modo claro',
                dark: 'Modo escuro'
            },
            help: "Ajuda",
            help_message: "Se você teve algum problema ou tem " +
            "alguma sugestão para melhorar o aplicativo, você pode " + 
            "ajudar criando uma <0 href='https://github.com/orangethewell/open-witness-library/issues/new' target='_blank'>Issue</0> " +
            "no <i>Github</i> explicando qual foi o seu problema ou " +
            "dando detalhes sobre sua sugestão."
        }
    }
}