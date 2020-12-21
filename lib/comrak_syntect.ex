defmodule ComrakSyntect do
  def hello do
    ComrakSyntect.Native.hello(100, 200)
  end

  def markdown_to_html(md, opts) do
    ComrakSyntect.Native.markdown_to_html(md, opts)
  end
end
