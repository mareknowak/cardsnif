defmodule Cardsnif.MixProject do
  use Mix.Project

  def project do
    [
      app: :cardsnif,
      version: "0.1.0",
      elixir: "~> 1.8",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: [:rustler] ++ Mix.compilers,
      rustler_crates: [
        cardsnif: [
          path: "native/cardsnif",
          mode: (if Mix.env == :prod, do: :release, else: :debug),
        ]
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
      {:rustler, "~> 0.20"},
      {:dialyxir, "~> 0.5", only: [:dev], runtime: false},
      {:exceptional, "~> 2.1"},
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
    ]
  end
end
