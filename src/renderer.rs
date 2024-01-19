use anyhow::Result as Anyhow;
use chrono::Datelike;
use genpdf::{
    elements::{Break, FrameCellDecorator, LinearLayout, Paragraph, TableLayout, Text},
    fonts::{FontData, FontFamily},
    style::Style,
    Document, Element,
};

use crate::prompt::Prompt;

// Constants
const IBM_PLEX_MONO_FONT_REGULAR: &'_ [u8] =
    include_bytes!("../resources/fonts/IBMPlexMono-Regular.ttf");
const IBM_PLEX_MONO_FONT_BOLD: &'_ [u8] = include_bytes!("../resources/fonts/IBMPlexMono-Bold.ttf");
const IBM_PLEX_MONO_FONT_ITALIC: &'_ [u8] =
    include_bytes!("../resources/fonts/IBMPlexMono-Italic.ttf");
const IBM_PLEX_MONO_FONT_BOLD_ITALIC: &'_ [u8] =
    include_bytes!("../resources/fonts/IBMPlexMono-BoldItalic.ttf");

pub(crate) fn render_to_pdf(prompt: &Prompt, path: impl AsRef<std::path::Path>) -> Anyhow<()> {
    let font = build_font_family()?;
    let mut doc = Document::new(font);
    doc.set_title("Demo document");

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins((30, 10, 30, 20));
    doc.set_page_decorator(decorator);
    doc.set_font_size(9);

    // header
    doc.push(
        Text::new(format!(
            "Faktura VAT / Invoice Nr. {}",
            generate_invoice_number(prompt)
        ))
        .styled(Style::new().bold().with_font_size(15)),
    );
    doc.push(Break::new(3));

    let split_lines = |text: String, prefix: &str| {
        text.lines()
            .fold(LinearLayout::vertical(), |mut layout, line| {
                layout.push(Paragraph::new(format!("{prefix}{line}")));
                layout
            })
    };

    // dates
    doc.push(split_lines(generate_dates(prompt), "  "));
    doc.push(Break::new(3));

    // supplier, customer
    let mut table = TableLayout::new(vec![1, 1]);

    let row = table.row();
    row.element(Text::new("Dane sprzedawcy / Supplier").styled(Style::new().bold()))
        .element(Text::new("Dane kupującego / Customer").styled(Style::new().bold()))
        .push()?;
    doc.push(table);
    doc.push(Break::new(1));

    let mut table = TableLayout::new(vec![1, 1]);
    let row = table.row();
    row.element(split_lines(generate_supplier(prompt), "  "))
        .element(split_lines(generate_customer(prompt), "  "))
        .push()?;
    doc.push(table);
    doc.push(Break::new(3));

    // details table
    let mut table = TableLayout::new(vec![5, 3, 2, 2, 2, 2]);
    table.set_cell_decorator(FrameCellDecorator::new(true, true, true));
    // header
    let label = |text: &str, is_bold: bool| {
        let mut style = Style::new();
        if is_bold {
            style.set_bold();
        }
        Paragraph::new(text)
            .aligned(genpdf::Alignment::Center)
            .styled(style)
            .padded((2, 2, 4, 2))
    };
    let row = table.row();
    row.element(label("Usługa / Service", true))
        .element(label("J.m.", true))
        .element(label("Netto, EUR", true))
        .element(label("VAT stawka", true))
        .element(label("VAT, PLN", true))
        .element(label("Brutto, EUR", true))
        .push()?;
    let row = table.row();
    row.element(label(&prompt.service, false))
        .element(label("usługa", false))
        .element(label(&format!("{:.2}", prompt.amount), false))
        .element(label("NP", false))
        .element(label("0.00", false))
        .element(label(&format!("{:.2}", prompt.amount), false))
        .push()?;
    doc.push(table);
    doc.push(Break::new(3));

    //  total
    doc.push(
        Text::new(format!(
            "  Kwota do zapłaty / Amount to pay: {:.2} EUR",
            prompt.amount
        ))
        .styled(Style::new().bold()),
    );

    // Render the document and write it to a file
    doc.render_to_file(path).expect("Failed to write PDF file");

    Ok(())
}

fn build_font_family() -> Anyhow<FontFamily<FontData>> {
    let regular = FontData::new(IBM_PLEX_MONO_FONT_REGULAR.to_vec(), None)?;
    let bold = FontData::new(IBM_PLEX_MONO_FONT_BOLD.to_vec(), None)?;
    let italic = FontData::new(IBM_PLEX_MONO_FONT_ITALIC.to_vec(), None)?;
    let bold_italic = FontData::new(IBM_PLEX_MONO_FONT_BOLD_ITALIC.to_vec(), None)?;
    Ok(FontFamily {
        regular,
        bold,
        italic,
        bold_italic,
    })
}

fn generate_invoice_number(prompt: &Prompt) -> String {
    format!(
        "{}/{}/{}",
        prompt.invoice_number,
        prompt.service_date.month(),
        prompt.service_date.year(),
    )
}

fn generate_dates(prompt: &Prompt) -> String {
    format!(
        "Data wystawienia / Issue date: {}\n\
        Data sprzedaży / Service date: {}\n\
        Termin zaplaty / Due date: {}",
        prompt.issue_date.format("%d-%m-%Y"),
        prompt.service_date.format("%d-%m-%Y"),
        prompt.due_date.format("%d-%m-%Y"),
    )
}

fn generate_supplier(prompt: &Prompt) -> String {
    format!(
        "{}\n{}\nNIP: {}\n\n{}",
        prompt.supplier.name,
        prompt.supplier.address,
        prompt.supplier.nip,
        generate_banking_details(prompt)
    )
}

fn generate_banking_details(prompt: &Prompt) -> String {
    format!(
        "Numer konta / Account:\n{}\nBank beneficjenta / Beneficiary bank:\n{}\nSWIFT: {}",
        prompt.account.number, prompt.account.bank, prompt.account.swift,
    )
}

fn generate_customer(prompt: &Prompt) -> String {
    format!("{}\n{}", prompt.customer.name, prompt.customer.address)
}
