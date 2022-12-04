local function lines_from(file)
    local lines = {}
    for line in io.lines(file) do
        lines[#lines + 1] = line
    end
    return lines
end

local function split(inputstr, sep)
    if sep == nil then
        sep = "%s"
    end
    local t={}
    for str in string.gmatch(inputstr, "([^"..sep.."]+)") do
        table.insert(t, str)
    end
    return t
end

local function range_contains(range, num)
    if tonumber(num) >= tonumber(range[1]) then
        if tonumber(num) <= tonumber(range[2]) then
            return true
        end
    end
    return false
end

local function part1(lines)
    local sum = 0
    for _, line in pairs(lines) do
        local left = split(line, ',')[1]
        local left_split = split(left, '-')
        local right = split(line, ',')[2]
        local right_split = split(right, '-')
        if (tonumber(left_split[1]) <= tonumber(right_split[1]) and tonumber(left_split[2]) >= tonumber(right_split[2]))
            or (tonumber(right_split[1]) <= tonumber(left_split[1]) and tonumber(right_split[2]) >= tonumber(left_split[2])) then
            sum = sum + 1
        end
    end
    return sum
end

local function part2(lines)
    local sum = 0
    for _, line in pairs(lines) do
        local left = split(line, ',')[1]
        local left_split = split(left, '-')
        local right = split(line, ',')[2]
        local right_split = split(right, '-')
        if (range_contains(right_split, left_split[1]) or range_contains(right_split, left_split[2]))
            or (range_contains(left_split, right_split[1]) or range_contains(left_split, right_split[2])) then
            sum = sum + 1
        end
    end
    return sum
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
