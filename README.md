# Email checker
Small cli program that checks if any new mail came along.

[Imap package](https://github.com/jonhoo/rust-imap) serves as based to create loop that will print number of unread messages. Program is used as part of [polybar](https://github.com/polybar/polybar) module.

## Example usage

After compiling program into binary:
```shell
./email-checker-rs --username a@a.com --password-file-path /$HOME/pass.txt --server imap.domain.com
```
