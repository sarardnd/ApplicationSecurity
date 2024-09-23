use yew::prelude::*;

#[function_component(Lobby)]
pub(crate) fn lobby() -> Html {
  html! {
      <>
      <head>
    <meta charset="UTF-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <title>{"Immagini con Descrizione e Casella di Testo"}</title>
    <link rel="stylesheet" href="styles.css"/>
</head>
<body>
    <div class="container">
        <div class="image-box">
            <img src="https://exclusion.it/wp-content/uploads/2024/02/cani-da-lavoro.jpeg" alt="Descrizione immagine 1"/>
            <p>{"Descrizione immagine 1"}</p>
            <textarea placeholder="Scrivi qui..."></textarea>
        </div>
        <div class="image-box">
            <img src="https://t4.ftcdn.net/jpg/03/83/25/83/360_F_383258331_D8imaEMl8Q3lf7EKU2Pi78Cn0R7KkW9o.jpg" alt="Descrizione immagine 2"/>
            <p>{"Descrizione immagine 2"}</p>
            <textarea placeholder="Scrivi qui..."></textarea>
        </div>
        <div class="image-box">
            <img src="https://hlloret.com/wp-content/uploads/2022/11/barcelona-casa-batllo-facade.jpg" alt="Descrizione immagine 3"/>
            <p>{"Descrizione immagine 3"}</p>
            <textarea placeholder="Scrivi qui..."></textarea>
        </div>
        <div class="image-box">
            <img src="https://media.istockphoto.com/id/155439315/it/foto/passeggero-aereo-volare-sopra-le-nuvole-durante-il-tramonto.jpg?s=612x612&w=0&k=20&c=eGlKEXjD9FoOH33EsnvcUEeEhaOqUGSEVURrxNvXiRU=" alt="Descrizione immagine 4"/>
            <p>{"Descrizione immagine 4"}</p>
            <textarea placeholder="Scrivi qui..."></textarea>
        </div>
    </div>
</body>
      </>

        /*<>
            <head>
                <meta charset="UTF-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <title>{"Images and test boxes"}</title>
                <link rel="stylesheet" href="webstyle.css"/>
            </head>
            <body>
                <div class="image-gallery">
                    <div class="column">
                        <img src="https://www.b2x.it/rest/images/2023/11/15/1488336.jpg?imageFormat=@1x" alt="Image 1"/>
                        <div class="caption">{"Image 1"}</div>
                        <input type="text" class="input-box" placeholder="Type here..."/>
                    </div>
                    <div class="column">
                        <img src="https://www.openpolis.it/wp-content/uploads/2024/03/etty-fidele-unsplash-migrant.jpg" alt="Image 2"/>
                        <div class="caption">{"Image 2"}</div>
                        <input type="text" class="input-box" placeholder="Type here..."/>
                    </div>
                    <div class="column">
                        <img src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQf_x5IBe875hDRs53e4JloCEoNYS-FEAIGCg&s" alt="Image 3"/>
                        <div class="caption">{"Image 3"}</div>
                        <input type="text" class="input-box" placeholder="Type here..."/>
                    </div>
                    <div class="column">
                        <img src="https://rockrun.com/cdn/shop/articles/bouldering-for-strength3_1600x.jpg?v=1602248316" alt="Image 4"/>
                        <div class="caption">{"Image 4"}</div>
                        <input type="text" class="input-box" placeholder="Type here..."/>
                    </div>
                </div>
            </body>
        </>*/
    }
}