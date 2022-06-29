#include <iostream>
#include <string>
#include "chasm.hpp"

/**
 * Example usage for how to call chasm
 */

// g++ main.c -L ./target/debug -lchasm -o example

int main(int argc, char const *argv[])
{
    int code = 0;
    std::string text = "main:\nAND R0, R0, #0\nADD R0, R0, #10";
    const chasm::TranslationOutput *intermediate = chasm::parse_asm(text.c_str(), &code);
    if (code != 0)
        return code;

    std::string second = "BR main";
    const chasm::TranslationOutput *stage_two = chasm::parse_asm_extend(second.c_str(), intermediate, &code);
    if (code != 0)
        return code;

    uintptr_t len;
    const uint16_t *assembly = chasm::link_asm(stage_two, &code, &len);

    if (code == 0)
    {
        for (size_t i = 0; i < len; i++)
        {
            printf("%04x ", *(assembly + i));
        }
        std::cout << std::endl;
    }
    else
    {
        std::cerr << "ERROR PARSING [" << code << ']' << std::endl;
    }

    return 0;
}
