use std::collections::HashSet;

use yew::prelude::*;

use crate::data::languages;

#[function_component(LanguagesPage)]
pub fn languages_page() -> Html {
    let cols = languages::columns();
    let rows = languages::rows();

    let collapsed_ids = use_state(|| {
        let mut set = HashSet::new();
        for col in cols.iter() {
            if col.default_collapsed {
                set.insert(col.id);
            }
        }
        set
    });

    let toggle_col = {
        let collapsed_ids = collapsed_ids.clone();
        Callback::from(move |col_id: &'static str| {
            let mut set = (*collapsed_ids).clone();
            if set.contains(col_id) {
                set.remove(col_id);
            } else {
                set.insert(col_id);
            }
            collapsed_ids.set(set);
        })
    };

    html! {
        <div class="page-section">
            <h1>{"Language Comparison"}</h1>
            <p class="lang-intro">
                {"Compare idioms across all COR24 languages side by side. \
                  Click any column header to collapse or expand it."}
            </p>
            <div class="lang-table-scroll">
                <table class="lang-table">
                    <thead>
                        <tr>
                            <th class="lang-th-label">{"Idiom"}</th>
                            {cols.iter().map(|col| {
                                let is_collapsed = collapsed_ids.contains(col.id);
                                render_header_cell(col, is_collapsed, &toggle_col)
                            }).collect::<Html>()}
                        </tr>
                    </thead>
                    <tbody>
                        {rows.iter().map(|row| {
                            html! {
                                <tr>
                                    <td class="lang-td-label">{row.label}</td>
                                    {cols.iter().map(|col| {
                                        let is_collapsed = collapsed_ids.contains(col.id);
                                        render_data_cell(row, col, is_collapsed)
                                    }).collect::<Html>()}
                                </tr>
                            }
                        }).collect::<Html>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

fn render_header_cell(
    col: &languages::LangColumn,
    is_collapsed: bool,
    toggle: &Callback<&'static str>,
) -> Html {
    let arrow = if is_collapsed { "\u{25BC}" } else { "\u{25B2}" };
    let onclick = {
        let toggle = toggle.clone();
        let col_id = col.id;
        Callback::from(move |_: MouseEvent| {
            toggle.emit(col_id);
        })
    };

    html! {
        <th
            class={classes!(
                "lang-th",
                "lang-th-code",
                "lang-col-expandable",
                if is_collapsed { "lang-col-collapsed" } else { "" }
            )}
            onclick={onclick}
        >
            <span class="lang-col-toggle">{arrow}</span>
            {col.label}
        </th>
    }
}

fn render_data_cell(
    row: &languages::IdiomRow,
    col: &languages::LangColumn,
    is_collapsed: bool,
) -> Html {
    if is_collapsed {
        html! { <td class="lang-td-code lang-col-collapsed"></td> }
    } else {
        let val = languages::cell_value(row, col.id);
        html! { <td class="lang-td-code"><code>{val}</code></td> }
    }
}
