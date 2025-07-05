INSERT INTO indexer.programs (program_id)
VALUES ('traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg');



INSERT INTO indexer.indexer (name, direction, program_id, finished, fetch_limit)
VALUES ('marketplace_up',
        'UP',
        'traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg',
        false,
        10);

INSERT INTO indexer.indexer (name, direction, program_id, finished, fetch_limit)
VALUES ('marketplace_down',
        'DOWN',
        'traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg',
        false,
        10);
