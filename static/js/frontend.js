// Set up the data model.
const current_book_details = {
    title: "Mithril Test Book",
    author: "Me",
    publisher: "Self-published",
    year: 2022,
    pages: 444,
    price: "0.00",
    cover: "Hard",
    series: "Lorem Ipsums",
    coverImg: "static/img/example.png",
    available: false,
    link: "http://goatse.com"
}
const CurrentBookAvailability = {
    view: () => {
        if (current_book_details.available) {
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
            m("h5", current_book_details.title.toLocaleUpperCase()),
            m("div.divider.my-3"),
            m("div.row", [
                m("div.col-3.m-auto", [
                    m("img.w-100", { src: current_book_details.coverImg })
                ]),
                m("div.col-5.row.m-auto", [
                    m("div.col-3.mr-5", [
                        m("div.row", [m("p", "Cena:")]),
                        m("div.row", [m("p", "Wydawnictwo:")])
                    ]),
                    m("div.col-2.ml-5", [
                        m("div.row", [m("h6.value", current_book_details.price + "zł")]),
                        m("div.row", [m("h6.value", current_book_details.publisher)])
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
                    m("div.row", [m("h6.value", current_book_details.author)]),
                    m("div.row", [m("h6.value", current_book_details.year)]),
                    m("div.row", [m("h6.value", current_book_details.pages)]),
                    m("div.row", [m("h6.value", current_book_details.cover)]),
                    m("div.row", [m("h6.value", current_book_details.series)]),
                ])
            ]),
            m("div.mt-4.row", [
                m("button.btn.pt-2.pb-2.button.mx-auto", { href: current_book_details.link, type: "button" }, "Przejdź do strony w sklepie")
            ]),
        ];
    }
}
// Get our mount points.
const infoArea = document.querySelector("div.infoarea");
// Start rendering.
m.mount(infoArea, CurrentBookComponent)
