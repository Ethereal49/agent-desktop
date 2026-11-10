use crate::sandbox::provider_spec::ProviderSpec;
use crate::{AdapterError, AppError, ErrorCode};

#[cfg(not(test))]
pub(crate) const PROVIDERS: &[ProviderSpec] = &[];

#[cfg(test)]
pub(crate) const PROVIDERS: &[ProviderSpec] = &[
    ProviderSpec {
        name: "stub",
        files: &[("provider.sh", "#!/bin/sh\nexit 0\n")],
    },
    ProviderSpec {
        name: "other",
        files: &[("provider.sh", "#!/bin/sh\nexit 0\n")],
    },
];

pub(crate) fn find_provider(name: &str) -> Result<&'static ProviderSpec, AppError> {
    PROVIDERS
        .iter()
        .find(|spec| spec.name == name)
        .ok_or_else(|| {
            let known: Vec<&str> = PROVIDERS.iter().map(|spec| spec.name).collect();
            let message = if known.is_empty() {
                format!(
                    "Unknown sandbox provider {name}. This build includes no sandbox providers."
                )
            } else {
                format!(
                    "Unknown sandbox provider {name}. Known providers: {}.",
                    known.join(", ")
                )
            };
            AppError::Adapter(
                AdapterError::new(ErrorCode::InvalidArgs, message).with_suggestion(
                    "Run `agent-desktop sandbox install` to see the providers and their states.",
                ),
            )
        })
}
