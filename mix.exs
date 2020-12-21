defmodule ComrakSyntect.MixProject do
  use Mix.Project

  def project do
    [
      app: :comrak_syntect,
      version: "0.1.0",
      elixir: "~> 1.10",
      start_permanent: Mix.env() == :prod,
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: rustler_crates(),
      deps: deps()
    ]
  end

  def rustler_crates do
    [
      comraksyntect: [
        path: "native/comraksyntect",
        cargo: :system,
        default_features: false,
        features: [],
        mode: :release,
        # mode: (if Mix.env == :prod, do: :release, else: :debug),
      ]
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.21.1"}
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
    ]
  end
end
