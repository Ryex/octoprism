
trait IntoOptionRef<'a, T> {
    fn to_option_ref(self) -> Option<&'a T>;
}

impl<'a, T> IntoOptionRef<'a, T> for &'a Option<T> {
    fn to_option_ref(self) -> Option<&'a T> {
        self.as_ref()
    }
}

impl<'a, T> IntoOptionRef<'a, T> for &'a Option<Box<T>> {
    fn to_option_ref(self) -> Option<&'a T> {
        self.as_ref().map(Box::as_ref)
    }
}

impl<'a, T> IntoOptionRef<'a, T> for &'a T {
    fn to_option_ref(self) -> Option<&'a T> {
        Some(self)
    }
}

trait IntoU64 {
    fn to_u64(self) -> u64;
}

impl IntoU64 for &u32 {
    fn to_u64(self) -> u64 {
        Into::into(*self)
    }
}

impl IntoU64 for &Option<u64> {
    fn to_u64(self) -> u64 {
        self.unwrap_or(0)
    }
}

macro_rules! pr_issue_map_field_fn {
    ($field_name:ident -><$fn:path> $return_type:ty) => {
        #[allow(dead_code)]
        pub fn $field_name(&self) -> $return_type {
            match self {
                Self::PullRequest(pr) => $fn(&pr.$field_name),
                Self::Issue(issue) => $fn(&issue.$field_name),
            }
        }
    };
    ($field_name:ident -> $return_type:ty) => {
        #[allow(dead_code)]
        pub fn $field_name(&self) -> $return_type {
            match self {
                Self::PullRequest(pr) => (&pr.$field_name).into(),
                Self::Issue(issue) => (&issue.$field_name).into(),
            }
        }
    };
    ($field_name:ident : $return_type:ty) => {
        #[allow(dead_code)]
        pub fn $field_name(&self) -> $return_type {
            match self {
                Self::PullRequest(pr) => pr.$field_name,
                Self::Issue(issue) => issue.$field_name,
            }
        }
    };
    ($field_name:ident &: $return_type:ty) => {
        #[allow(dead_code)]
        pub fn $field_name(&self) -> $return_type {
            match self {
                Self::PullRequest(pr) => &pr.$field_name,
                Self::Issue(issue) => &issue.$field_name,
            }
        }
    };
}

macro_rules! pr_issue_map_fields {
    () => {};
    ($field_name:ident : $return_type:ty $(, $($tail:tt)*)?) => {
        pr_issue_map_field_fn!{$field_name : $return_type}
        $(pr_issue_map_fields!{$($tail)*})?
    };
    ($field_name:ident &: $return_type:ty $(, $($tail:tt)*)?) => {
        pr_issue_map_field_fn!{$field_name &: $return_type}
        $(pr_issue_map_fields!{$($tail)*})?
    };
    ($field_name:ident -><$fn:path> $return_type:ty $(, $($tail:tt)*)?) => {
        pr_issue_map_field_fn!{$field_name -><$fn> $return_type}
        $(pr_issue_map_fields!{$($tail)*})?
    };
    ($field_name:ident -> $return_type:ty $(, $($tail:tt)*)?) => {
        pr_issue_map_field_fn!{$field_name -> $return_type}
        $(pr_issue_map_fields!{$($tail)*})?
    };
}

#[allow(clippy::large_enum_variant)]
pub enum PullRequestOrIssue {
    PullRequest(octocrab::models::pulls::PullRequest),
    Issue(octocrab::models::issues::Issue),
}

impl PullRequestOrIssue {
    pr_issue_map_fields! {
        node_id -> Option<&String>,
        user -><IntoOptionRef::to_option_ref> Option<&octocrab::models::Author>,
        url -><ToString::to_string> String,
        comments_url -> Option<&url::Url>,
        html_url -> Option<&url::Url>,
        number : u64,
        comments -><IntoU64::to_u64> u64,
        state -> Option<&octocrab::models::IssueState>,
        title -> Option<&String>,
        body -> Option<&String>,
        body_text -> Option<&String>,
        body_html -> Option<&String>,
        labels -> Option<&Vec<octocrab::models::Label>>,
        assignees -> Option<&Vec<octocrab::models::Author>>,
        locked : bool,
        created_at -> Option<&chrono::DateTime<chrono::Utc>>,
        updated_at -> Option<&chrono::DateTime<chrono::Utc>>,
        closed_at -> Option<&chrono::DateTime<chrono::Utc>>,
        milestone -><IntoOptionRef::to_option_ref> Option<&octocrab::models::Milestone>,
    }
}

impl std::convert::From<octocrab::models::pulls::PullRequest> for PullRequestOrIssue {
    fn from(value: octocrab::models::pulls::PullRequest) -> Self {
        PullRequestOrIssue::PullRequest(value)
    }
}

impl std::convert::From<octocrab::models::issues::Issue> for PullRequestOrIssue {
    fn from(value: octocrab::models::issues::Issue) -> Self {
        PullRequestOrIssue::Issue(value)
    }
}
