use super::{MAX_PIPE_NAME_UNITS, PeerCredentialError, bounded_pipe_name};
use std::cell::Cell;

#[test]
fn accepts_local_utf16_name_and_appends_one_terminator() -> Result<(), PeerCredentialError> {
    let source = r"\\.\pipe\intent-é-🙂";
    let encoded = bounded_pipe_name(source.encode_utf16())?;
    assert_eq!(
        encoded,
        source.encode_utf16().chain([0]).collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn rejects_nonlocal_empty_embedded_nul_and_nested_names() {
    for name in [
        "",
        r"C:\pipe",
        r"\\server\pipe\name",
        r"\\.\pipe\",
        "\\\\.\\pipe\\a\0b",
        r"\\.\pipe\a\b",
    ] {
        assert_eq!(
            bounded_pipe_name(name.encode_utf16()),
            Err(PeerCredentialError::InvalidPipeName)
        );
    }
}

#[test]
fn counts_utf16_units_at_the_name_limit() -> Result<(), PeerCredentialError> {
    let prefix = r"\\.\pipe\";
    let name = format!(
        "{prefix}{}🙂",
        "a".repeat(MAX_PIPE_NAME_UNITS - prefix.len() - 2)
    );
    let encoded = bounded_pipe_name(name.encode_utf16())?;
    assert_eq!(encoded.len(), MAX_PIPE_NAME_UNITS + 1);
    assert_eq!(
        bounded_pipe_name(format!("{name}a").encode_utf16()),
        Err(PeerCredentialError::InvalidPipeName)
    );
    Ok(())
}

#[test]
fn oversized_name_stops_before_traversing_or_allocating_the_full_input() {
    let consumed = Cell::new(0);
    let units = r"\\.\pipe\"
        .encode_utf16()
        .chain(std::iter::repeat_n(u16::from(b'x'), 16_384));
    let units = units.inspect(|_| consumed.set(consumed.get() + 1));
    assert_eq!(
        bounded_pipe_name(units),
        Err(PeerCredentialError::InvalidPipeName)
    );
    assert_eq!(consumed.get(), MAX_PIPE_NAME_UNITS + 1);
}
