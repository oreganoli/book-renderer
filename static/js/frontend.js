// Set up the data model.
// All books:
var books = [];
const processBooks = () => {
    books.forEach((book) => {
        if (book.data.year == -1) {
            book.data.year = "-"
        }
        if (book.data.pages == -1) {
            book.data.pages = "-"
        }
        if (book.data.price == "-1.00") {
            book.data.price = "-"
        }
        if (book.data.cover == "") {
            book.data.cover = "-"
        }
        if (book.data.series == "") {
            book.data.series = "-"
        }
    });
};
// Book currently viewed in the details pane:
var curr_book = null; //{
//     available: false,
//     data: {
//         title: "Mithril Test Book",
//         author: "Me",
//         publisher: "Self-published",
//         year: 2022,
//         pages: 444,
//         price: "0.00",
//         cover: "Hard",
//         series: "Lorem Ipsums",
//         shop_url: "http://goatse.com"
//     },
//     description: "-",
//     link_img: "static/img/example.png",
// }
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
        if (curr_book == null) {
            return m("h5", "Nie wybrano żadnej książki");
        }
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
                m("button.btn.pt-2.pb-2.button.mx-auto", {
                    href: curr_book.data.shop_url, type: "button", onclick: () => {
                        window.location.href = curr_book.link;
                    }
                }, "Przejdź do strony w sklepie")
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
        return m("div.ml-5", {
            onclick: () => {
                curr_book = book;
                m.redraw();
            }
        }, [
            m("article.mt-4.mb-4.row", [
                m("button.row.col.12", { type: "button", onclick: () => { } }, [
                    m("div.col-2", [m("img.w-100", { src: book.link_img })]),
                    m("div.col-7.mt-3", [
                        m("span", {
                            style: "color: #000",
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
// Make the search form use our API.
let searchForm = document.getElementById("searchForm");
searchForm.addEventListener("submit", (e) => {
    e.preventDefault();
    let url_params = new URLSearchParams();
    if (searchForm.elements.title_contains.value != "") {
        url_params.append("title_contains", searchForm.elements.title_contains.value)
    }
    if (searchForm.elements.min_price.value != "") {
        url_params.append("min_price", searchForm.elements.min_price.value)
    }
    if (searchForm.elements.max_price.value != "") {
        url_params.append("max_price", searchForm.elements.max_price.value)
    }
    if (searchForm.elements.sort_by.value != "") {
        url_params.append("sort_by", searchForm.elements.sort_by.value)
    }
    m.request({
        method: "GET",
        url: "/api/books?" + url_params.toString()
    }).then((data) => {
        books = data;
        processBooks();
        if (books.length > 0) {
            curr_book = books[0];
        } else {
            curr_book = null;
        }
    })
})

// Get our mount points.
const infoArea = document.querySelector("div.infoarea");
const scrollArea = document.querySelector("div.scrollarea");
// Start rendering.
m.mount(infoArea, CurrentBookComponent);
m.mount(scrollArea, BookTable);
// Populate books:
m.request({
    method: "GET",
    url: "/api" + window.location.pathname + window.location.search,
}).then((data) => {
    books = data;
    processBooks();
    if (books.length > 0) {
        curr_book = books[0];
    } else {
        curr_book = null;
    }
    m.redraw();
    //console.log(data);
});