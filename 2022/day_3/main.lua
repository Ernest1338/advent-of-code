local function lines_from(file)
    local lines = {}
    for line in io.lines(file) do
        lines[#lines + 1] = line
    end
    return lines
end

local function table_contains(table, search)
    for _, value in pairs(table) do
        if value == search then
            return true
        end
    end
    return false
end

local function table_index(table, search)
    for i, value in pairs(table) do
        if value == search then
            return i
        end
    end
    return nil
end

local function string_contains(string, search)
    for char in string:gmatch(".") do
        if char == search then
            return true
        end
    end
    return false
end

local function value_of(item)
    local lower = {
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    };
    local upper = {
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    };
    if table_contains(lower, item) then
        return table_index(lower, item)
    elseif table_contains(upper, item) then
        return table_index(upper, item) + 26
    end
end

local function part1(lines)
    local sum = 0
    for _, line in pairs(lines) do
        if line == "" then
            break
        end
        local line_len_half = #line / 2
        local first_half = string.sub(line, 1, line_len_half)
        local second_half = string.sub(line, line_len_half + 1, #line)
        for item in first_half:gmatch(".") do
            if string_contains(second_half, item) then
                sum = sum + value_of(item)
                break
            end
        end
    end
    print(sum)
end

local function part2(lines)
    local sum = 0
    for i=1,#lines,3 do
        local first = lines[i];
        local second = lines[i + 1];
        local third = lines[i + 2];
        for item in first:gmatch(".") do
            if string_contains(second, item) then
                if string_contains(third, item) then
                    sum = sum + value_of(item)
                    break
                end
            end
        end
    end
    print(sum)
end

local function main()
    local part = 2

    local file_lines = lines_from("input.txt")
    if part == 1 then
        part1(file_lines)
    else
        part2(file_lines)
    end

end

main()
