#[allow(unused)]
use crate::Table;
use crate::TableOption;
use papergrid::{Border, Entity, Grid, Settings};

/// Style is responsible for a look of a [Table].
///
/// # Example
///
/// ```rust,no_run
/// use tabled::{Table, Style, style::Line};
/// let data = vec!["Hello", "2021"];
/// let table = Table::new(&data).with(
///                 Style::NO_BORDER
///                     .frame_bottom(Some(Line::short('*', ' ')))
///                     .split(Some(Line::short('*', ' ')))
///                     .inner(' ')
///             )
///             .to_string();
///
/// println!("{}", table);
/// ```
pub struct Style {
    frame: Frame,
    header_split_line: Option<Line>,
    split: Option<Line>,
    inner_split_char: char,
}

impl Style {
    /// Default style looks like the following table
    ///
    /// ```text
    ///     +----+--------------+---------------------------+
    ///     | id | destribution |           link            |
    ///     +----+--------------+---------------------------+
    ///     | 0  |    Fedora    |  https://getfedora.org/   |
    ///     +----+--------------+---------------------------+
    ///     | 2  |   OpenSUSE   | https://www.opensuse.org/ |
    ///     +----+--------------+---------------------------+
    ///     | 3  | Endeavouros  | https://endeavouros.com/  |
    ///     +----+--------------+---------------------------+
    /// ```
    pub const ASCII: Self = Self::new(
        Frame {
            bottom: Some(Line::bordered('-', '+', '+', '+')),
            top: Some(Line::bordered('-', '+', '+', '+')),
            left: Some('|'),
            right: Some('|'),
        },
        Some(Line::bordered('-', '+', '+', '+')),
        Some(Line::bordered('-', '+', '+', '+')),
        '|',
    );

    /// Noborder style looks like the following table
    ///
    /// ```text
    ///      id   destribution             link
    ///      0       Fedora       https://getfedora.org/
    ///      2      OpenSUSE     https://www.opensuse.org/
    ///      3    Endeavouros    https://endeavouros.com/
    /// ```
    pub const NO_BORDER: Self = Self::new(Frame::empty(), None, None, ' ');

    /// Psql style looks like the following table
    ///
    /// ```text
    ///      id | destribution |           link
    ///     ----+--------------+---------------------------
    ///      0  |    Fedora    |  https://getfedora.org/
    ///      2  |   OpenSUSE   | https://www.opensuse.org/
    ///      3  | Endeavouros  | https://endeavouros.com/
    /// ```
    pub const PSQL: Self = Self::new(Frame::empty(), Some(Line::short('-', '+')), None, '|');

    /// Github_markdown style looks like the following table
    ///
    /// ```text
    ///     | id | destribution |           link            |
    ///     |----+--------------+---------------------------|
    ///     | 0  |    Fedora    |  https://getfedora.org/   |
    ///     | 2  |   OpenSUSE   | https://www.opensuse.org/ |
    ///     | 3  | Endeavouros  | https://endeavouros.com/  |
    /// ```
    pub const GITHUB_MARKDOWN: Self = Self::new(
        Frame {
            left: Some('|'),
            right: Some('|'),
            bottom: None,
            top: None,
        },
        Some(Line::bordered('-', '+', '|', '|')),
        None,
        '|',
    );

    /// Pseudo style looks like the following table
    ///
    /// ```text
    ///     ┌────┬──────────────┬───────────────────────────┐
    ///     │ id │ destribution │           link            │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 0  │    Fedora    │  https://getfedora.org/   │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 2  │   OpenSUSE   │ https://www.opensuse.org/ │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 3  │ Endeavouros  │ https://endeavouros.com/  │
    ///     └────┴──────────────┴───────────────────────────┘
    /// ```
    pub const PSEUDO: Self = Self::new(
        Frame {
            left: Some('│'),
            right: Some('│'),
            bottom: Some(Line::bordered('─', '┴', '└', '┘')),
            top: Some(Line::bordered('─', '┬', '┌', '┐')),
        },
        Some(Line::bordered('─', '┼', '├', '┤')),
        Some(Line::bordered('─', '┼', '├', '┤')),
        '│',
    );

    /// Pseudo_clean style looks like the following table
    ///
    /// ```text
    ///     ┌────┬──────────────┬───────────────────────────┐
    ///     │ id │ destribution │           link            │
    ///     ├────┼──────────────┼───────────────────────────┤
    ///     │ 0  │    Fedora    │  https://getfedora.org/   │
    ///     │ 2  │   OpenSUSE   │ https://www.opensuse.org/ │
    ///     │ 3  │ Endeavouros  │ https://endeavouros.com/  │
    ///     └────┴──────────────┴───────────────────────────┘
    /// ```
    pub const PSEUDO_CLEAN: Self = Self::new(
        Frame {
            left: Some('│'),
            right: Some('│'),
            bottom: Some(Line::bordered('─', '┴', '└', '┘')),
            top: Some(Line::bordered('─', '┬', '┌', '┐')),
        },
        Some(Line::bordered('─', '┼', '├', '┤')),
        None,
        '│',
    );

    #[deprecated(note = "The name is not explicit. Use ASCII constant instead.")]
    pub fn default() -> Self {
        Self::ASCII
    }

    #[deprecated(note = "The name is not explicit. Use ASCII constant instead.")]
    pub fn ascii() -> Self {
        Self::ASCII
    }

    #[deprecated(note = "The name is not explicit. Use NO_BORDER constant instead.")]
    pub fn noborder() -> Self {
        Self::NO_BORDER
    }

    #[deprecated(note = "The name is not explicit. Use PSQL constant instead.")]
    pub fn psql() -> Self {
        Self::PSQL
    }

    #[deprecated(note = "The name is not explicit. Use GITHUB_MARKDOWN constant instead.")]
    pub fn github_markdown() -> Self {
        Self::GITHUB_MARKDOWN
    }

    #[deprecated(note = "The name is not explicit. Use PSEUDO constant instead.")]
    pub fn pseudo() -> Self {
        Self::PSEUDO
    }

    #[deprecated(note = "The name is not explicit. Use PSEUDO_CLEAN constant instead.")]
    pub fn pseudo_clean() -> Self {
        Self::PSEUDO_CLEAN
    }

    /// Left frame character.
    pub fn frame_left(mut self, frame: Option<char>) -> Self {
        self.frame.left = frame;
        self
    }

    /// Right frame character.
    pub fn frame_right(mut self, frame: Option<char>) -> Self {
        self.frame.right = frame;
        self
    }

    /// The header's top line.
    ///
    /// It's suppose that [Self::frame_bottom] and [Self::split]  has the same type of [Line] short or bordered.  
    pub fn frame_top(mut self, frame: Option<Line>) -> Self {
        self.frame.top = frame;
        self
    }

    /// The footer's bottom line.
    ///
    /// It's suppose that [Self::frame_top] and [Self::split] has the same type of [Line] short or bordered.
    pub fn frame_bottom(mut self, frame: Option<Line>) -> Self {
        self.frame.bottom = frame;
        self
    }

    /// The header's bottom line.
    pub fn header(mut self, line: Option<Line>) -> Self {
        self.header_split_line = line;
        self
    }

    /// Row split line.
    ///
    /// [Self::frame_top] and [Self::frame_bottom]
    pub fn split(mut self, line: Option<Line>) -> Self {
        self.header_split_line = line.clone();
        self.split = line;
        self
    }

    /// Inner split character.
    pub fn inner(mut self, c: char) -> Self {
        self.inner_split_char = c;
        self
    }

    const fn new(frame: Frame, header: Option<Line>, split: Option<Line>, inner: char) -> Self {
        Self {
            frame,
            split,
            header_split_line: header,
            inner_split_char: inner,
        }
    }
}

/// Line represents a horizontal line on a [Table].
#[derive(Debug, Clone, Default)]
pub struct Line {
    main: char,
    intersection: char,
    left_corner: Option<char>,
    right_corner: Option<char>,
}

impl Line {
    /// A line for frame styles.
    pub const fn bordered(main: char, intersection: char, left: char, right: char) -> Self {
        Self {
            intersection,
            main,
            left_corner: Some(left),
            right_corner: Some(right),
        }
    }

    /// A line for no-frame styles.
    pub const fn short(main: char, intersection: char) -> Self {
        Self {
            main,
            intersection,
            left_corner: None,
            right_corner: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Frame {
    top: Option<Line>,
    bottom: Option<Line>,
    left: Option<char>,
    right: Option<char>,
}

impl Frame {
    const fn empty() -> Self {
        Self {
            bottom: None,
            top: None,
            left: None,
            right: None,
        }
    }
}

impl TableOption for Style {
    fn change(&mut self, grid: &mut Grid) {
        grid.clear_split_grid();

        let count_rows = grid.count_rows();
        let count_columns = grid.count_columns();
        for row in 0..count_rows {
            for column in 0..count_columns {
                let border = make_style(
                    self,
                    row == 0,
                    row + 1 == count_rows,
                    column == 0,
                    column + 1 == count_columns,
                );

                grid.set(
                    &Entity::Cell(row, column),
                    Settings::default().border(border).border_restriction(false),
                );
            }
        }
    }
}

fn make_style(
    style: &Style,
    is_first_row: bool,
    is_last_row: bool,
    is_first_column: bool,
    is_last_column: bool,
) -> Border {
    match (is_first_row, is_last_row, is_first_column, is_last_column) {
        // A table with a single cell
        (true, true, true, true) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            right: style.frame.right,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.right_corner),
            left: style.frame.left,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.left_corner),
        },
        // Single row
        (true, true, true, false) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.left_corner),
            right: Some(style.inner_split_char),
            right_top_corner: style.frame.top.as_ref().map(|l| l.intersection),
            right_bottom_corner: style.frame.bottom.as_ref().map(|l| l.intersection),
        },
        (true, true, false, true) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.frame.top.as_ref().map(|l| l.intersection),
            left_bottom_corner: style.frame.bottom.as_ref().map(|l| l.intersection),
            right: Some(style.inner_split_char),
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.right_corner),
        },
        (true, true, false, false) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: Some(style.inner_split_char),
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.frame.bottom.as_ref().map(|l| l.intersection),
            right: Some(style.inner_split_char),
            right_top_corner: style.frame.top.as_ref().map(|l| l.intersection),
            right_bottom_corner: style.frame.bottom.as_ref().map(|l| l.intersection),
        },
        // Single column
        (true, false, true, true) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.header_split_line.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.header_split_line.as_ref().and_then(|l| l.left_corner),
            right: style.frame.right,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style
                .header_split_line
                .as_ref()
                .and_then(|l| l.right_corner),
        },
        (false, true, true, true) => Border {
            top: style.frame.bottom.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.frame.bottom.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.left_corner),
            right: style.frame.right,
            right_top_corner: style.frame.bottom.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.right_corner),
        },
        (false, false, true, true) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.split.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.split.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.split.as_ref().and_then(|l| l.left_corner),
            right: style.frame.right,
            right_top_corner: style.split.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.split.as_ref().and_then(|l| l.right_corner),
        },
        // General
        (true, false, true, false) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.header_split_line.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.frame.top.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.header_split_line.as_ref().and_then(|l| l.left_corner),
            right: Some(style.inner_split_char),
            right_top_corner: style.frame.top.as_ref().map(|l| l.intersection),
            right_bottom_corner: style.header_split_line.as_ref().map(|l| l.intersection),
        },
        (true, false, false, true) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.header_split_line.as_ref().map(|l| l.main),
            left: Some(style.inner_split_char),
            left_top_corner: style.frame.top.as_ref().map(|l| l.intersection),
            left_bottom_corner: style.header_split_line.as_ref().map(|l| l.intersection),
            right: style.frame.right,
            right_top_corner: style.frame.top.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style
                .header_split_line
                .as_ref()
                .and_then(|l| l.right_corner),
        },
        (true, false, false, false) => Border {
            top: style.frame.top.as_ref().map(|l| l.main),
            bottom: style.header_split_line.as_ref().map(|l| l.main),
            left: Some(style.inner_split_char),
            left_top_corner: style.frame.top.as_ref().map(|l| l.intersection),
            left_bottom_corner: style.header_split_line.as_ref().map(|l| l.intersection),
            right: Some(style.inner_split_char),
            right_top_corner: style.frame.top.as_ref().map(|l| l.intersection),
            right_bottom_corner: style.header_split_line.as_ref().map(|l| l.intersection),
        },
        (false, true, true, false) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.split.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.left_corner),
            right: Some(style.inner_split_char),
            right_top_corner: style.split.as_ref().map(|l| l.intersection),
            right_bottom_corner: style.frame.bottom.as_ref().map(|l| l.intersection),
        },
        (false, true, false, true) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: Some(style.inner_split_char),
            left_top_corner: style.split.as_ref().map(|l| l.intersection),
            left_bottom_corner: style.frame.bottom.as_ref().map(|l| l.intersection),
            right: style.frame.right,
            right_top_corner: style.split.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.frame.bottom.as_ref().and_then(|l| l.right_corner),
        },
        (false, true, false, false) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.frame.bottom.as_ref().map(|l| l.main),
            left: Some(style.inner_split_char),
            left_top_corner: style.split.as_ref().map(|l| l.intersection),
            left_bottom_corner: style.frame.bottom.as_ref().map(|l| l.intersection),
            right: Some(style.inner_split_char),
            right_top_corner: style.split.as_ref().map(|l| l.intersection),
            right_bottom_corner: style.frame.bottom.as_ref().map(|l| l.intersection),
        },
        (false, false, true, false) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.split.as_ref().map(|l| l.main),
            left: style.frame.left,
            left_top_corner: style.split.as_ref().and_then(|l| l.left_corner),
            left_bottom_corner: style.split.as_ref().and_then(|l| l.left_corner),
            right: Some(style.inner_split_char),
            right_top_corner: style.split.as_ref().map(|l| l.intersection),
            right_bottom_corner: style.split.as_ref().map(|l| l.intersection),
        },
        (false, false, false, true) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.split.as_ref().map(|l| l.main),
            left: Some(style.inner_split_char),
            left_top_corner: style.split.as_ref().map(|l| l.intersection),
            left_bottom_corner: style.split.as_ref().map(|l| l.intersection),
            right: style.frame.right,
            right_top_corner: style.split.as_ref().and_then(|l| l.right_corner),
            right_bottom_corner: style.split.as_ref().and_then(|l| l.right_corner),
        },
        (false, false, false, false) => Border {
            top: style.split.as_ref().map(|l| l.main),
            bottom: style.split.as_ref().map(|l| l.main),
            left: Some(style.inner_split_char),
            left_top_corner: style.split.as_ref().map(|l| l.intersection),
            left_bottom_corner: style.split.as_ref().map(|l| l.intersection),
            right: Some(style.inner_split_char),
            right_top_corner: style.split.as_ref().map(|l| l.intersection),
            right_bottom_corner: style.split.as_ref().map(|l| l.intersection),
        },
    }
}
