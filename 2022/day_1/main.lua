local function lines_from(file)
    local lines = {}
        for line in io.lines(file) do
            lines[#lines + 1] = line
        end
    return lines
end

local function part1(lines)
    local sum, highest = 0, 0
    for _, line in pairs(lines) do
        if line ~= "" then
            sum = sum + line
        else
            if sum > highest then
                highest = sum
            end
            sum = 0
        end
    end
    print(highest)
end

local function part2(lines)
    local sum, highest = 0, {}
    for _, line in pairs(lines) do
        if line ~= "" then
            sum = sum + line
        else
            table.insert(highest, sum)
            sum = 0
        end
    end
    table.insert(highest, sum)
    table.sort(highest)
    local final = highest[#highest] + highest[#highest-1] + highest[#highest-2]
    print(final)
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
