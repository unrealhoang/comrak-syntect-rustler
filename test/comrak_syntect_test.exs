defmodule ComrakSyntectTest do
  use ExUnit.Case
  doctest ComrakSyntect

  test "greets the world" do
    assert ComrakSyntect.hello() == {:ok, 300}
  end

  test "markdown_to_html workds" do
    options = %ComrakSyntect.Native.Options{extension: %ComrakSyntect.Native.ExtensionOptions{strikethrough: true}}
    assert ComrakSyntect.markdown_to_html("Hello, **世界**!", options) == {:ok, "<p>Hello, <strong>世界</strong>!</p>\n"}
  end

  test "markdown_to_html highlights code" do
    options = %ComrakSyntect.Native.Options{
      extension: %ComrakSyntect.Native.ExtensionOptions{strikethrough: true},
      render: %ComrakSyntect.Native.RenderOptions{unsafe_: true}
    }
    str = """
    # Rust code
    ```rust
    fn hello(args: Vec<u8>) {
        println!("Args: {}", args);
    }
    ```
    """
    {:ok, result} = ComrakSyntect.markdown_to_html(str, options)
    assert result =~ "span"
    assert result =~ "rust"
  end
end
