defmodule RomanConverter do
  @map %{"I" => 1, "V" => 5, "X" => 10, "L" => 50, "C" => 100, "D" => 500, "M" => 1000}

  def main do
    input = IO.gets("Введите римское число: ")
    case input do
      nil -> :ok
      _ ->
        s = String.trim(input)
        if String.match?(s, ~r/^[IVXLCDMivxlcdm]+$/) do
          IO.puts("Результат: #{calc(s)}")
        else
          IO.puts("Ошибка! Только римские цифры (I,V,X,L,C,D,M). Повтор: ")
          main()
        end
    end
  end

  defp calc(s) do
    s
    |> String.upcase()
    |> String.graphemes()
    |> Enum.map(&@map[&1])
    |> do_calc(0)
  end

  defp do_calc([], acc), do: acc
  defp do_calc([x], acc), do: acc + x
  defp do_calc([x, y | rest], acc) do
    if x < y do
      do_calc([y | rest], acc - x)
    else
      do_calc([y | rest], acc + x)
    end
  end
end

RomanConverter.main()