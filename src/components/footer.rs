use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="site-footer">
            <div class="footer-inner">
                <span class="footer-item">{"\u{00a9} 2026 Michael A Wright"}</span>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <span class="footer-item">{"MIT License"}</span>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://makerlisp.com" target="_blank" class="footer-item footer-link">
                    {"makerlisp.com"}
                </a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://github.com/sw-embed" target="_blank" class="footer-item footer-link">
                    {"GitHub"}
                </a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <span class="footer-item footer-dim">{env!("BUILD_SHA")}</span>
            </div>
        </footer>
    }
}
