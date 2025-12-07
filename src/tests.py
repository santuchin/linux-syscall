
file = open('sys.csv', 'r')
raw = file.read()
file.close()

sysnums = (line.split(',') for line in raw.strip().splitlines())
formatted = '\n'.join(f'{sysnum[2]} = {sysnum[0]};' for sysnum in sysnums)

file = open('sys2.rs', 'w')
file.write(formatted)
file.close()
