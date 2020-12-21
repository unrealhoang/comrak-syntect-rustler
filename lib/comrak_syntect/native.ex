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

defmodule ComrakSyntect.Native.Options do
  defstruct [
    extension: %ComrakSyntect.Native.ExtensionOptions{}
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

