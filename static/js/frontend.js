// Set up the data model.
// Book currently viewed in the details pane:
const curr_book = {
    available: false,
    data: {
        title: "Mithril Test Book",
        author: "Me",
        publisher: "Self-published",
        year: 2022,
        pages: 444,
        price: "0.00",
        cover: "Hard",
        series: "Lorem Ipsums",
        shop_url: "http://goatse.com"
    },
    description: "-",
    link_img: "static/img/example.png",
}
const CurrentBookAvailability = {
    view: () => {
        if (curr_book.available) {
            return m("p.availability", { style: "color: #12D251" }, "Dostępny");
        } else {
            return m("p.availability", { style: "color: #E13521" }, "Niedostępny");
        }
    }
}
// This component displays the current book's detailed view.
const CurrentBookComponent = {
    view: () => {
        return [
            m("h5", curr_book.data.title),
            m("div.divider.my-3"),
            m("div.row", [
                m("div.col-3.m-auto", [
                    m("img.w-100", { src: curr_book.link_img })
                ]),
                m("div.col-5.row.m-auto", [
                    m("div.col-3.mr-5", [
                        m("div.row", [m("p", "Cena:")]),
                        m("div.row", [m("p", "Wydawnictwo:")])
                    ]),
                    m("div.col-2.ml-5", [
                        m("div.row", [m("h6.value", curr_book.data.price.replace(".", ",") + "zł")]),
                        m("div.row", [m("h6.value", curr_book.data.publisher)])
                    ])
                ])]),
            m("div.divider.my-3"),
            m("div.row.mx-auto", [
                m("div.col-4.m-auto.p-auto", [
                    m("div.row", [m("p", "Autor:")]),
                    m("div.row", [m("p", "Rok wydania:")]),
                    m("div.row", [m("p", "Liczba stron:")]),
                    m("div.row", [m("p", "Okładka:")]),
                    m("div.row", [m("p", "Seria:")]),
                ]),
                m("div.col-3.m-auto.p-auto", [
                    m("div.row", [m("h6.value", curr_book.data.author)]),
                    m("div.row", [m("h6.value", curr_book.data.year)]),
                    m("div.row", [m("h6.value", curr_book.data.pages)]),
                    m("div.row", [m("h6.value", curr_book.data.cover)]),
                    m("div.row", [m("h6.value", curr_book.data.series)]),
                ])
            ]),
            m(CurrentBookAvailability),
            m("div.mt-4.row", [
                m("button.btn.pt-2.pb-2.button.mx-auto", { href: curr_book.data.shop_url, type: "button" }, "Przejdź do strony w sklepie")
            ]),
        ];
    }
}
// Get our mount points.
const infoArea = document.querySelector("div.infoarea");
// Start rendering.
m.mount(infoArea, CurrentBookComponent)
