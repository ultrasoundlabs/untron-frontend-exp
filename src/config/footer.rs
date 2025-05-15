pub struct FooterLink {
    pub text: &'static str,
    pub href: &'static str,
}

pub struct FooterSection {
    pub title: &'static str,
    pub links: &'static [FooterLink],
}

pub const FOOTER_LINKS: &[FooterSection] = &[
    FooterSection {
        title: "Project",
        links: &[
            FooterLink {
                text: "Blog",
                href: "https://x.com/untronfi",
            },
            FooterLink {
                text: "About us",
                href: "https://x.com/untronfi",
            },
            FooterLink {
                text: "Terms of service",
                href: "https://www.wtfpl.net/wp-content/uploads/2012/12/freedom.jpeg",
            },
            FooterLink {
                text: "Brand assets",
                href: "https://github.com/ultrasoundlabs/brandkit",
            },
        ],
    },
    FooterSection {
        title: "Socials",
        links: &[
            FooterLink {
                text: "X / Twitter",
                href: "https://x.com/untronfi",
            },
            FooterLink {
                text: "Telegram",
                href: "https://t.me/untronchat",
            },
            FooterLink {
                text: "GitHub",
                href: "https://github.com/ultrasoundlabs",
            },
        ],
    },
    FooterSection {
        title: "Contacts",
        links: &[
            FooterLink {
                text: "SHPS (LLC) Ultrasound Labs",
                href: "mailto:contact@untron.finance",
            },
            FooterLink {
                text: "contact@untron.finance",
                href: "mailto:contact@untron.finance",
            },
        ],
    },
];
