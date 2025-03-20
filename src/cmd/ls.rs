use chrono::{DateTime, Local};
use colored::*;
use prettytable::{format, row, Table};
use std::{
  fs,
  os::unix::fs::{MetadataExt as _, PermissionsExt},
  time::SystemTime,
};

pub fn ls(path: &str) {
  let args = path.split_whitespace().collect::<Vec<&str>>();

  if args.len() > 2 {
    println!("Usage: ls [path]");
    println!("By default the path is the current directory");
    return;
  }

  let path = path.split_whitespace().nth(1).unwrap_or(".");

  if let Some(_) = fs::metadata(path).err() {
    println!("{} does not exist", path);
    return;
  }

  let entries = fs::read_dir(path).expect("Failed to read directory");
  let mut entries_vec: Vec<_> = entries.filter_map(Result::ok).collect();
  entries_vec.sort_by_key(|e| e.file_name());

  let mut table = Table::new();

  // Customize table format to be more compact and consistent
  let format = format::FormatBuilder::new()
    .column_separator(' ')
    .borders(' ')
    .separators(&[], format::LineSeparator::new('-', ' ', ' ', ' '))
    .padding(1, 1)
    .build();
  table.set_format(format);

  // Collect all entries first to determine appropriate column widths
  let mut rows = Vec::new();
  for entry in entries_vec {
    let file_name = entry.file_name();
    let metadata = entry.metadata().unwrap();
    let permissions = format!("{:o}", metadata.permissions().mode());
    let perm_str = if permissions.len() >= 6 {
      &permissions[permissions.len() - 3..]
    } else {
      &permissions
    };

    let file_type = entry.file_type().unwrap();
    let is_dir = file_type.is_dir();
    let name_colored = if is_dir {
      file_name.to_string_lossy().blue().bold().to_string()
    } else if metadata.permissions().mode() & 0o111 != 0 {
      file_name.to_string_lossy().green().to_string()
    } else {
      file_name.to_string_lossy().to_string()
    };

    let size_str = if metadata.size() < 1024 {
      format!("{}B", metadata.size())
    } else if metadata.size() < 1024 * 1024 {
      format!("{:.1}K", metadata.size() as f64 / 1024.0)
    } else {
      format!("{:.1}M", metadata.size() as f64 / (1024.0 * 1024.0))
    };

    let created = format_system_time(metadata.created().unwrap());
    let modified = format_system_time(metadata.modified().unwrap());
    let type_str = if is_dir {
      "Directory".blue()
    } else {
      "File".normal()
    };

    rows.push((
      format_permissions(perm_str).to_string(),
      size_str,
      created,
      modified,
      name_colored,
      type_str.to_string(),
    ));
  }

  // Add header row with proper alignment
  table.add_row(row![
    bFr->"Perms",
    bFr->"Size",
    bFc->"Created",
    bFc->"Modified",
    bFl->"Name",
    bFl->"Type"
  ]);

  // Add data rows
  for (perms, size, created, modified, name, type_str) in rows {
    table.add_row(row![
      Fr->perms,
      Fr->size,
      Fc->created,
      Fc->modified,
      Fl->name,
      Fl->type_str
    ]);
  }

  table.printstd();
}

fn format_permissions(perm_str: &str) -> ColoredString {
  let mut result = String::new();
  for (i, c) in perm_str.chars().enumerate() {
    let _mask = match i {
      0 => 'r',
      1 => 'w',
      2 => 'x',
      _ => '-',
    };

    if c == '0' {
      result.push('-');
    } else if c == '7' {
      result.push_str("rwx");
      break;
    } else {
      // Convert octal digit to rwx representation
      let digit = c.to_digit(8).unwrap_or(0);
      result.push(if digit & 4 != 0 { 'r' } else { '-' });
      result.push(if digit & 2 != 0 { 'w' } else { '-' });
      result.push(if digit & 1 != 0 { 'x' } else { '-' });
      break;
    }
  }

  result.yellow()
}

fn format_system_time(time: SystemTime) -> String {
  let datetime: DateTime<Local> = time.into();
  datetime.format("%Y-%m-%d %H:%M").to_string() // Shorter time format
}
