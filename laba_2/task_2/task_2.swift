import Foundation
func romanToInt(_ s: String) -> Int {
    let roman: [Character: Int] = [
        "I": 1, "V": 5, "X": 10, "L": 50,
        "C": 100, "D": 500, "M": 1000
    ]
    var result = 0
    let values = s.uppercased().compactMap { roman[$0] }
    for (i, val) in values.enumerated() {
        if i < values.count - 1, val < values[i + 1] {
            result -= val
        } else {
            result += val
        }
    }
    return result
}

func main() {
    let allowed = CharacterSet(charactersIn: "IVXLCDMivxlcdm")
    
    while true {
        print("Введите римское число: ", terminator: "")
        guard let input = readLine()?.trimmingCharacters(in: .whitespacesAndNewlines) else { return }
        
        if input.isEmpty { continue }
        
        if input.rangeOfCharacter(from: allowed.inverted) == nil {
            print("Результат: \(romanToInt(input))")
            break
        }
        print("Ошибка! Только римские цифры (I,V,X,L,C,D,M). Повтор: ")
    }
}

main()