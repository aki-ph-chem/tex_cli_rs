use argh::FromArgs;
pub mod mkdir;

#[derive(FromArgs, Clone)]
/// Struct to get authour name, title from command line option
pub struct CmdOpt {
    /// project name
    #[argh(positional)]
    pub project: Option<String>,

    /// authour name
    #[argh(option, short = 'a', default = "String::from(\"Authour\")")]
    pub authour: String,

    /// title
    #[argh(option, short = 't', default = "String::from(\"Title\")")]
    pub title: String,
}

/// Struct to hold author name and title
#[derive(Debug)]
pub struct Paper {
    /// authour name
    authour: String,

    /// tittle of paper
    title: String,
}

impl Paper {
    /// constructor
    pub fn new(cmd_ops: &CmdOpt) -> Paper {
        Paper {
            authour: cmd_ops.authour.clone(),
            title: cmd_ops.title.clone(),
        }
    }

    /// set author name and title to LaTex template from struct Paper
    pub fn gen_latex_template(&self) -> String {
        let author = &self.authour;
        let title = &self.title;

        format!(
            r#"\documentclass[dvipdfmx]{{jsarticle}}

% アンカーを作る
%\usepackage[dvipdfmx]{{hyperref}}
%\usepackage{{pxjahyper}}

\begin{{document}}
\title{{{}}}
\author{{{}}}
\thispagestyle{{empty}}
\maketitle

% 目次
%\tableofcontents
%\clearpage
\end{{document}}"#,
            title, author
        )
    }

    /// generate Makefile for build pdf document
    pub fn gen_makfile() -> String {
        r#"TEX = platex
PDF = dvipdfmx
f = main

viewer = qpdfview
#viewer = mupdf

${f}.pdf : ${f}.dvi
	${PDF} $<

#${f}.dvi : ${f}.tex
#	${TEX} $< 
${f}.dvi : ${f}.tex
	@(${TEX} -interaction=nonstopmode $< > /dev/null 2>&1); \
		if [ $$? -eq 0 ]; then \
		echo "compile 1 is successed!"; \
		else \
		echo -e "failure! please read ${f}.log"; \
                exit 1;\
		fi
	@((grep -q "Rerun to get" ${f}.log || [ -f ${f}.toc ]) && platex -interaction=nonstopmode $< > /dev/null 2>&1); \
		if [ $$? -eq 0 ]; then \
		echo "compile 2 is successed!"; \
		fi

opn :
	${viewer} ${f}.pdf &

clean: 
	rm *.aux *.dvi *.log *.pdf *.toc *.out

.PHONY : opn clean
        "#.to_string()
    }

    pub fn gen_build_sh() -> String {
        r#"#!/bin/sh
f='main'

# generate dvi
platex -interaction=nonstopmode "$f".tex
if [ $? -eq 0 ]; then
    echo "compile 1 is successed!";
else
    echo -e "failure! in compile 1, please read ${f}.log";
    exit 1
fi

grep -q "Rerun to get" ${f}.log || [ -f ${f}.toc ] && platex -interaction=nonstopmode "$f".tex 
if [ $? -eq 0 ]; then 
    echo "compile 2 is successed!";
fi

# generate pdf
dvipdfmx "$f".dvi
        "#.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_latex_template() {
        let ans = r#"\documentclass[dvipdfmx]{jsarticle}

% アンカーを作る
%\usepackage[dvipdfmx]{hyperref}
%\usepackage{pxjahyper}

\begin{document}
\title{Card Shop}
\author{Jouji Handa}
\thispagestyle{empty}
\maketitle

% 目次
%\tableofcontents
%\clearpage
\end{document}"#
            .to_string();
        let paper = Paper {
            authour: "Jouji Handa".to_string(),
            title: "Card Shop".to_string(),
        };
        let result = paper.gen_latex_template();

        assert_eq!(ans, result);
    }

    #[test]
    fn test_gen_makefile() {
        let result = Paper::gen_makfile();
        println!("{}", result);
    }
}
