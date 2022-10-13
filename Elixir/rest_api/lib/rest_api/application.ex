defmodule RestApi.Application do
  use Application

  # The @impl true here denotes that the start function is implementing a
  # callback that was defined in the Application module
  # https://hexdocs.pm/elixir/main/Module.html#module-impl
  # This will aid the compiler to warn you when a implementaion is incorrect
  @impl true
  def start(_type, _args) do
    children = [
      {Plug.Cowboy, scheme: :http, plug: RestApi.Router, options: [port: 8080]}
    ]

    opts = [strategy: :one_for_one, name: RestApi.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
