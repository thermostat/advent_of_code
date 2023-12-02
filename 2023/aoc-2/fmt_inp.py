
import json

def fmt(inf, outf):
    fd = open(inf, 'r')
    data = fd.readlines()
    fd.close()

    gamelist = []
    

    for line in data:
        if line.strip() == '':
            continue
        game_id, plays = line.strip().split(':', 2)
        game_nbr = int(game_id.split(' ')[1].strip())
        game_json = {}
        plays_json = []
        for play in plays.split(';'):
            for pull in play.strip().split(','):
                count, color = pull.strip().split(' ', 2)
                count = int(count.strip())
                color = color.strip()
                plays_json.append((count, color))
        game_json['id'] = game_nbr
        game_json['plays'] = plays_json
        gamelist.append(game_json)
    fd = open(outf, 'w')
    json.dump(gamelist, fd)
    fd.close()

if __name__ == '__main__':
    import sys
    fmt(sys.argv[1], f'{sys.argv[1]}.json')
    
