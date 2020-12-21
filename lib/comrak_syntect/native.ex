defmodule NifNotLoadedError do
  defexception message: "nif not loaded"
end

defmodule ComrakSyntect.Native do
  use Rustler, otp_app: :comrak_syntect, crate: :comraksyntect

  def hello(_a, _b), do: err()

  defp err() do
    throw NifNotLoadedError
  end
end

