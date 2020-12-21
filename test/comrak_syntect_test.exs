defmodule ComrakSyntectTest do
  use ExUnit.Case
  doctest ComrakSyntect

  test "greets the world" do
    assert ComrakSyntect.hello() == {:ok, 300}
  end
end
