local function lines_from(file)
  local lines = {}
  for line in io.lines(file) do
    lines[#lines + 1] = line
  end
  return lines
end

local function part1(lines)

end

local function part2(lines)

end

local function main()
    local file_name = "example.txt"
    local lines = lines_from(file_name)

    local part = 1

    if part == 1 then
        part1(lines)
    elseif part == 2 then
        part2(lines)
    end
end

main()
