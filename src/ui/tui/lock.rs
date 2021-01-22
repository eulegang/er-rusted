#[derive(Debug, PartialEq)]
pub enum WindowLock {
    Top,
    Perc20,
    Middle,
    Perc80,
    Bottom,
}

impl WindowLock {
    /// finds the position to start rendering or err if negative space should be used
    pub fn find_pos(&self, height: usize, cur: usize) -> Result<usize, usize> {
        let diff = match self {
            WindowLock::Top => 1,
            WindowLock::Perc20 => height / 5,
            WindowLock::Middle => height / 2,
            WindowLock::Perc80 => height - (height / 5),
            WindowLock::Bottom => height,
        };

        if let Some(s) = cur.checked_sub(diff) {
            Ok(s)
        } else {
            Err(diff - cur)
        }
    }

    pub fn next(&self) -> WindowLock {
        match self {
            WindowLock::Top => WindowLock::Perc20,
            WindowLock::Perc20 => WindowLock::Middle,
            WindowLock::Middle => WindowLock::Perc80,
            WindowLock::Perc80 => WindowLock::Bottom,
            WindowLock::Bottom => WindowLock::Top,
        }
    }

    pub fn prev(&self) -> WindowLock {
        match self {
            WindowLock::Top => WindowLock::Bottom,
            WindowLock::Perc20 => WindowLock::Top,
            WindowLock::Middle => WindowLock::Perc20,
            WindowLock::Perc80 => WindowLock::Middle,
            WindowLock::Bottom => WindowLock::Perc80,
        }
    }
}
