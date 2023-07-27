use html_node::{html, text, unsafe_text};

fn main() {
    let evil = "</script><script>alert('evil')</script>";

    let safe_html = html! {
        <div>
            <p>{text!("{evil}")}</p>
        </div>
    };

    let unsafe_html = html! {
        <div>
            <p>{unsafe_text!("{evil}")}</p>
        </div>
    };

    println!("--- safe ---\n{safe_html:#}");
    println!();
    println!("--- unsafe ---\n{unsafe_html:#}");
}
