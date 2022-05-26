// Set up the data model.
// All books:
var books = [];
// Book currently viewed in the details pane:
var curr_book = {
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
const BookPriceAvailability = {
    view: (vnode) => {
        let price = vnode.attrs.price;
        let available = vnode.attrs.available;
        if (available) {
            return [
                m("h5", price.replace(".", ",") + " zł"),
                m("p.availability", { style: "color: #12D251" }, "Dostępny")
            ];
        } else {
            return m("p.availability", { style: "color: #E13521" }, "Niedostępny");
        }
    }
};
const BookRow = {
    view: (vnode) => {
        let book = vnode.attrs.book;
        return m("div.ml-5", [
            m("article.mt-4.mb-4.row", [
                m("button.row.col.12", { type: "button", onclick: () => { } }, [
                    m("div.col-2", [m("img.w-100", { src: book.link_img })]),
                    m("div.col-7.mt-3", [
                        m("a", {
                            style: "color: #000",
                            href: book.link
                        }, [m("h5", { style: "font-weight: 600;" }, book.data.title)]),
                        m("p", {
                            style: "font-size: 13; font-weight: 400;"
                        },
                            [m("b", "Autor: " + book.data.author)])
                    ]),
                    m("div.col-3.mt-3", [
                        m(BookPriceAvailability, { price: book.data.price, available: book.available })
                    ]),
                    m("div.divider.mt-4")
                ])
            ])
        ]);
    }
};
const BookTable = {
    view: () => {
        return books.map((book) => m(BookRow, { book: book }));
    }
}

// Get our mount points.
const infoArea = document.querySelector("div.infoarea");
const scrollArea = document.querySelector("div.scrollarea");
// Start rendering.
m.mount(infoArea, CurrentBookComponent);
m.mount(scrollArea, BookTable);
// Populate books:
m.request({
    method: "GET",
    url: "/api" + window.location.pathname,
}).then((data) => {
    books = data;
    curr_book = books[0];
    m.redraw();
    //console.log(data);
});