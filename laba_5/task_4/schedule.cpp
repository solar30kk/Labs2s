    #include "schedule.h"
#include <iostream>
#include <algorithm>

using namespace std;

ScheduleManager::ScheduleManager() : currentMonth(1), daysInMonth({
    {1, 31}, {2, 28}, {3, 31}, {4, 30},
    {5, 31}, {6, 30}, {7, 31}, {8, 31},
    {9, 30}, {10, 31}, {11, 30}, {12, 31}
}) {}

Command ScheduleManager::parseCommand(const string& cmd) const {
    if (cmd == "CLASS") return Command::CLASS;
    if (cmd == "NEXT") return Command::NEXT;
    if (cmd == "VIEW") return Command::VIEW;
    return Command::UNKNOWN;
}

int ScheduleManager::getDaysInCurrentMonth() const {
    auto it = daysInMonth.find(currentMonth);
    return (it != daysInMonth.end()) ? it->second : 31;
}

void ScheduleManager::addClass(int day, const string& discipline) {
    int days = getDaysInCurrentMonth();
    
    if (day < 1 || day > days) {
        cout << "Ошибка: В этом месяце всего " << days << " дней" << endl;
        return;
    }
    
    auto& dayDisciplines = schedule[currentMonth][day];
    
    for (const auto& d : dayDisciplines) {
        if (d == discipline) {
            cout << "Ошибка: Дисциплина " << discipline << " уже есть в этот день" << endl;
            return;
        }
    }
    
    dayDisciplines.push_back(discipline);
    cout << "Добавлена дисциплина " << discipline << " на день " << day << endl;
}

void ScheduleManager::nextMonth() {
    int oldMonth = currentMonth;
    int oldMonthDays = getDaysInCurrentMonth();
    
    // Переходим на следующий месяц
    currentMonth = (currentMonth % 12) + 1;
    int newMonthDays = getDaysInCurrentMonth();
    
    cout << "Переход на следующий месяц" << endl;
    
    // Если новый месяц короче старого, нужно перенести "хвост"
    if (newMonthDays < oldMonthDays) {
        auto& lastDayOfNewMonth = schedule[currentMonth][newMonthDays];
        
        // Собираем все дела из дней, которых нет в новом месяце
        for (int d = newMonthDays + 1; d <= oldMonthDays; ++d) {
            auto it = schedule[oldMonth].find(d);
            if (it != schedule[oldMonth].end()) {
                for (const string& discipline : it->second) {
                    // Переносим только если такой дисциплины в последний день еще нет
                    if (find(lastDayOfNewMonth.begin(), lastDayOfNewMonth.end(), discipline) == lastDayOfNewMonth.end()) {
                        lastDayOfNewMonth.push_back(discipline);
                    }
                }
            }
        }
    }
}

void ScheduleManager::viewDay(int day) const {
    int days = getDaysInCurrentMonth();
    
    if (day < 1 || day > days) {
        cout << "Ошибка: В этом месяце всего " << days << " дней" << endl;
        return;
    }
    
    auto monthIt = schedule.find(currentMonth);
    if (monthIt == schedule.end()) {
        cout << "В день " << day << " мы свободны!" << endl;
        return;
    }
    
    auto dayIt = monthIt->second.find(day);
    if (dayIt == monthIt->second.end() || dayIt->second.empty()) {
        cout << "В день " << day << " мы свободны!" << endl;
        return;
    }
    
    cout << "В день " << day << " занятия в университете: ";
    bool first = true;
    for (const auto& discipline : dayIt->second) {
        if (!first) cout << ", ";
        cout << discipline;
        first = false;
    }
    cout << endl;
}
