use sycamore::prelude::*;

#[component(Header<G>)]
pub fn header() -> Template<G> {
    template! {
        header {
            nav(class="navbar navbar-expand-sm navbar-dark bg-dark") {
                div(class="container-fluid") {
                    a(class="navbar-brand", href="/#") { "Sycamore" }

                    ul(class="navbar-nav") {
                        li(class="nav-item") {
                            a(class="nav-link", href="/getting_started/installation") { "Book" }
                        }
                        li(class="nav-item") {
                            a(class="nav-link", href="https://docs.rs/sycamore") { "API" }
                        }
                        li(class="nav-item") {
                            a(class="nav-link", href="https://github.com/sycamore-rs/sycamore") { "Repository" }
                        }
                    }
                }
            }
        }
    }
}
