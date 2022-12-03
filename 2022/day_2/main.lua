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

local function part1(lines)
    local score = 0
    for _, line in pairs(lines) do
        local oponnent = split(line, " ")[1]
        local response = split(line, " ")[2]
        -- print(oponnent .. "\t" .. response)
        if oponnent == 'A' then
            -- rock
            if response == 'X' then
                -- rock
                score = score + 1
                -- draw
                score = score + 3
            elseif response == 'Y' then
                -- paper
                score = score + 2
                -- win
                score = score + 6
            elseif response == 'Z' then
                -- scissors
                score = score + 3
                -- loose
            end
        elseif oponnent == 'B' then
            -- paper
            if response == 'X' then
                -- rock
                score = score + 1
                -- loose
            elseif response == 'Y' then
                -- paper
                score = score + 2
                -- draw
                score = score + 3
            elseif response == 'Z' then
                -- scissors
                score = score + 3
                -- win
                score = score + 6
            end
        elseif oponnent == 'C' then
            -- scissors
            if response == 'X' then
                -- rock
                score = score + 1
                -- win
                score = score + 6
            elseif response == 'Y' then
                -- paper
                score = score + 2
                -- loose
            elseif response == 'Z' then
                -- scissors
                score = score + 3
                -- draw
                score = score + 3
            end
        end
    end
    print(score)
end

local function part2(lines)
    local score = 0
    for _, line in pairs(lines) do
        local oponnent = split(line, " ")[1]
        local response = split(line, " ")[2]
        -- print(oponnent .. "\t" .. response)
        if oponnent == 'A' then
            -- rock
            if response == 'X' then
                -- need to loose
                -- use: scissors
                score = score + 3
                -- loose
            elseif response == 'Y' then
                -- need to draw
                -- use: rock
                score = score + 1
                -- draw
                score = score + 3
            elseif response == 'Z' then
                -- need to win
                -- use: paper
                score = score + 2
                -- win
                score = score + 6
            end
        elseif oponnent == 'B' then
            -- paper
            if response == 'X' then
                -- need to loose
                -- use: rock
                score = score + 1
                -- loose
            elseif response == 'Y' then
                -- need to draw
                -- use: paper
                score = score + 2
                -- draw
                score = score + 3
            elseif response == 'Z' then
                -- need to win
                -- use: scissors
                score = score + 3
                -- win
                score = score + 6
            end
        elseif oponnent == 'C' then
            -- scissors
            if response == 'X' then
                -- need to loose
                -- use: paper
                score = score + 2
                -- loose
            elseif response == 'Y' then
                -- need to draw
                -- use: scissors
                score = score + 3
                -- draw
                score = score + 3
            elseif response == 'Z' then
                -- need to win
                -- use: rock
                score = score + 1
                -- win
                score = score + 6
            end
        end
    end
    print(score)
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
