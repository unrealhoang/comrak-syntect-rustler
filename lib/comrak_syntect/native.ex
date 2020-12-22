defmodule NifNotLoadedError do
  defexception message: "nif not loaded"
end

defmodule ComrakSyntect.Native.ExtensionOptions do
  defstruct [
    strikethrough: false,
    tagfilter: false,
    table: false,
    autolink: false,
    tasklist: false,
    superscript: false,
    header_ids: nil,
    footnotes: false,
    description_lists: false
  ]
end

defmodule ComrakSyntect.Native.ParseOptions do
  defstruct [
    smart: false,
    default_info_string: nil,
  ]
end

defmodule ComrakSyntect.Native.RenderOptions do
  defstruct [
    hardbreaks: false,
    github_pre_lang: false,
    width: 0,
    unsafe_: false,
    escape: false
  ]
end

defmodule ComrakSyntect.Native.Options do
  defstruct [
    extension: %ComrakSyntect.Native.ExtensionOptions{},
    parse: %ComrakSyntect.Native.ParseOptions{},
    render: %ComrakSyntect.Native.RenderOptions{},
  ]
end

defmodule ComrakSyntect.Native do
  use Rustler, otp_app: :comrak_syntect, crate: :comraksyntect

  def hello(_a, _b), do: err()
  def markdown_to_html(_md, _opts), do: err()

  defp err() do
    throw NifNotLoadedError
  end
end

