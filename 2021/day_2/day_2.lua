function lines_from(file)
  local lines = {}
  for line in io.lines(file) do 
    lines[#lines + 1] = line
  end
  return lines
end

function part1(lines)
    local h_pos, depth = 0, 0
    for _,l in pairs(lines) do
        local instruction = {}
        for element in l:gmatch("%w+") do table.insert(instruction, element) end
        if instruction[1] == "forward" then h_pos = h_pos + instruction[2]
        elseif instruction[1] == "down" then depth = depth + instruction[2]
        elseif instruction[1] == "up" then depth = depth - instruction[2]
        end
    end
    print("Answer: "..h_pos*depth)
end

function part2(lines)
    local h_pos, depth, aim = 0, 0, 0
    for _,l in pairs(lines) do
        local instruction = {}
        for element in l:gmatch("%w+") do table.insert(instruction, element) end
        if instruction[1] == "forward" then
            h_pos = h_pos + instruction[2]
            depth = depth + aim * instruction[2]
        elseif instruction[1] == "down" then aim = aim + instruction[2]
        elseif instruction[1] == "up" then aim = aim - instruction[2]
        end
    end
    print("Answer: "..h_pos*depth)
end

function main()
    local file_name = "input.txt"
    local lines = lines_from(file_name)

    local part = 2

    if part == 1 then
        part1(lines)
    elseif part == 2 then
        part2(lines)
    end
end

main()
