defmodule ComrakSyntectTest do
  use ExUnit.Case
  doctest ComrakSyntect

  test "greets the world" do
    assert ComrakSyntect.hello() == {:ok, 300}
  end

  test "markdown_to_html workds" do
    options = %Elixir.ComrakSyntect.Native.Options{extension: %Elixir.ComrakSyntect.Native.ExtensionOptions{strikethrough: true}}
    assert ComrakSyntect.markdown_to_html("Hello, **世界**!", options) == {:ok, "<p>Hello, <strong>世界</strong>!</p>\n"}
  end
end
