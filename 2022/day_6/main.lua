local function lines_from(file)
    local lines = {}
    for line in io.lines(file) do
        lines[#lines + 1] = line
    end
    return lines
end

local function to_char_table(str)
    local chars = {}
    for char in str:gmatch(".") do
        table.insert(chars, char)
    end
    return chars
end

local function table_dedup(tab)
    local res, hash = {}, {}
    for _, v in ipairs(tab) do
        if (not hash[v]) then
            res[#res + 1] = v
            hash[v] = true
        end
    end
    return res
end

local function table_windows(tab, num)
    local res = {}
    for i = 1, #tab - num + 1 do
        local tmp = {}
        for a = i, i + num - 1 do
            table.insert(tmp, tab[a])
        end
        table.insert(res, tmp)
    end
    return res
end

local function all_different(table)
    local before_len = #table
    table = table_dedup(table)
    if #table == before_len then
        return true
    end
    return false
end

local function find_header(file_lines, num_of_unique)
    local chars = to_char_table(file_lines[1])
    for i, tab in pairs(table_windows(chars, num_of_unique)) do
        if all_different(tab) then
            return i + num_of_unique - 1
        end
    end
    return 0
end

local function part1(file_lines)
    return find_header(file_lines, 4)
end

local function part2(file_lines)
    return find_header(file_lines, 14)
end

local function main()
    local part = 2

    local file_lines = lines_from("input.txt")
    if part == 1 then
        print("Answer: " .. part1(file_lines))
    else
        print("Answer: " .. part2(file_lines))
    end
end

main()
