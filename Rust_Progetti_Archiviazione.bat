		@REM non ci deve essere spazio altrimenti il nome del file viene con lo spazio SI=%PATH_ARRIVO_s%%%~nG ... NO=%PATH_ARRIVO_s% %%~nG
		
		rem c:\Casa\RAR.EXE a -r -U /D c:\CASA\PROGRAMMI\RUST_PROGETTI\AA_SALVATAGGI\RustSorgenti.rar *capitolo_* 
		rem c:\Casa\RAR.EXE a c:\CASA\PROGRAMMI\RUST_PROGETTI\AA_SALVATAGGI\RustSorgenti.rar capitolo_01 capitolo_02
		
		c:\Casa\RAR.EXE a -r -u c:\CASA\PROGRAMMI\RUST_PROGETTI\AA_SALVATAGGI\RustSorgenti.rar @Rust_Progetti_Lista.lst
		
		@REM COPIA NELLA CARTELLA c:\CASA\PROGRAMMI\GIT_DESKTOP\CASA\PROGRAMMI\RUST_PROGETTI\
		
		XCOPY /Y c:\CASA\PROGRAMMI\RUST_PROGETTI\AA_SALVATAGGI\RustSorgenti.rar c:\CASA\PROGRAMMI\GIT_DESKTOP\CASA\PROGRAMMI\RUST_PROGETTI\

	REM			SOSPENSIONE
	@REM -----------------------------------------------------------------------------------------
	
		@REM 01)
		@REM sospensione per 1 secondo con crononometro 
		@REM timeout /t 2 /nobreak > NUL
		@REM @choice /C:X /N /T:10 /D:X > NUL
		@REM ---->	TIMEOUT /T 10 /NOBREAK
		
		@REM Ritardo per 3 secondi
		@choice /C:X /N /T:3 /D:X > NUL

	
	
	@REM			SOSPENSIONE *** FINE ***
	@REM -----------------------------------------------------------------------------------------




@REM //NOTE DI FUNZIONAMENTO
@REM //============================================================================//
@REM attenzione per il salvataggio sono stati utilizzati questi parametri:
@REM dir 	/b = elenca il contenuto della cartella corrente senza dettagli
@REM 	/s = Mostra il percorso completo di ogni file/cartella E DELLE SOTTOCARTELLE!!!!
@REM 	/a:d = include SOLO LE CARTELLE (escluso perche altrimenti salvava ogni singolo
@REM 	file della sottocartella in un zip)
@REM COMANDO rar
@REM rar.exe	-ep1= esclude la path nel .zip e inserisce solo il nome della sottocartella nell'archivio.	

