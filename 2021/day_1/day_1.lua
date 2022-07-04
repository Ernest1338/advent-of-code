function lines_from(file)
  local lines = {}
  for line in io.lines(file) do 
    lines[#lines + 1] = line
  end
  return lines
end

function part1(lines)
    local count = 0
    local previous = lines[1]
    for i,l in pairs(lines) do
        if tonumber(l) > tonumber(previous) then
            count = count + 1
        end
        previous = l
    end
    print("Answer: "..count)
end

function part2(lines)
    local count = 0
    local previous = lines[1]+lines[2]+lines[3]
    for i=2,#lines-2 do
        local current = lines[i]+lines[i+1]+lines[i+2]
        if current > previous then
            count = count + 1
        end
        previous = current
    end
    print("Answer: "..count)
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
