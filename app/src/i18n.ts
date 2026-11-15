export const languages = ["en", "de", "fr", "es", "pt"] as const;
export type Language = (typeof languages)[number];

export const languageNames: Record<Language, string> = {
  en: "English",
  de: "Deutsch",
  fr: "Français",
  es: "Español",
  pt: "Português",
};

const messages = {
  en: {
    tagline: "YOUR GAMING PROGRESS. ONE PLACE.", localOnly: "LOCAL ONLY", steamLibrary: "STEAM LIBRARY",
    headline: "Your games, at a glance.", intro: "GameVault checks local Steam installations and shows which data is available on this PC.",
    scan: "Scan Steam locally", scanning: "Scanning…", overview: "Overview", localReadOnly: "Local detection · Read only",
    gamesFound: "{count} local games", notScanned: "Not scanned yet", waitingTitle: "Your library is waiting",
    waitingText: "Start a local scan. GameVault does not change Steam files and does not need your credentials.",
    steamInstall: "STEAM INSTALLATION", found: "Found", notFound: "Not found", commonLocations: "Common install locations checked",
    steamSession: "STEAM SESSION", notConnected: "Not connected", sessionDetail: "Connects only for a requested read",
    achievements: "ACHIEVEMENTS", dataRead: "Data read", unavailable: "Unavailable", notRead: "Not read",
    achievementsDetail: "Available on request for individual installed games", incompleteScan: "Some local entries could not be checked because of access restrictions or safety limits.",
    inspectError: "Could not check Steam folders.", libraryTitle: "Local game library", installed: "installed locally",
    loadAchievements: "Show achievements", reloadAchievements: "Read again", loading: "Reading…",
    earned: "unlocked · read directly from the Steam client", noAchievements: "Steam reports no achievements for this game.",
    steamRequestNote: "Steam processes this request for your signed-in account. Experimental client interface · Read only",
    prototype: "Experimental local prototype", windowsSteam: "GameVault · Windows · Steam",
    detectedNote: "Installed local manifests are candidates only; this is not a complete owned library. Steam is contacted only when you explicitly request achievement data.",
    noSteam: "Steam installation was not found in the usual locations.",
    workerCrashed: "The isolated Steam reader crashed (code {code}); GameVault remained separate. Last recorded step: {stage}.",
    errSteamLibrary: "The Steam client library could not be loaded.", errInterfaces: "The expected Steam read interfaces are unavailable.",
    errStatsInterface: "Steam does not provide the requested SteamUserStats013 interface.", errUserInterface: "Steam does not provide the requested SteamUser012 interface.",
    errSession: "No signed-in Steam session was found.", errTimeout: "Steam did not answer the achievement request in time.",
    errNotInstalled: "This game is not installed in a locally checked Steam library.", errInvalid: "The Steam request or app ID is invalid.",
    errReaderStart: "The isolated Steam reader could not be started.", errReaderReply: "The isolated Steam reader returned an invalid or incomplete response.",
    errDefault: "Achievement data could not be read.",
    language: "Language", achievedDate: "Unlocked {date}",
  },
  de: {
    tagline: "DEIN SPIELFORTSCHRITT. AN EINEM ORT.", localOnly: "NUR LOKAL", steamLibrary: "STEAM-BIBLIOTHEK",
    headline: "Deine Spiele, auf einen Blick.", intro: "GameVault prüft lokale Steam-Installationen und zeigt, welche Daten auf diesem PC verfügbar sind.",
    scan: "Steam lokal prüfen", scanning: "Wird geprüft …", overview: "Übersicht", localReadOnly: "Lokale Erkennung · Nur lesend",
    gamesFound: "{count} lokale Spiele", notScanned: "Noch nicht geprüft", waitingTitle: "Deine Bibliothek wartet",
    waitingText: "Starte die lokale Prüfung. GameVault verändert keine Steam-Dateien und benötigt keine Zugangsdaten.",
    steamInstall: "STEAM-INSTALLATION", found: "Gefunden", notFound: "Nicht gefunden", commonLocations: "Übliche Installationsorte wurden geprüft",
    steamSession: "STEAM-SITZUNG", notConnected: "Nicht verbunden", sessionDetail: "Verbindung nur während einer angeforderten Leseaktion",
    achievements: "ERRUNGENSCHAFTEN", dataRead: "Daten gelesen", unavailable: "Nicht verfügbar", notRead: "Nicht gelesen",
    achievementsDetail: "Für einzelne installierte Spiele auf Anfrage lesbar", incompleteScan: "Einige lokale Einträge konnten wegen Zugriffsbeschränkungen oder Sicherheitslimits nicht geprüft werden.",
    inspectError: "Steam-Verzeichnisse konnten nicht geprüft werden.", libraryTitle: "Lokale Spielebibliothek", installed: "lokal installiert",
    loadAchievements: "Errungenschaften anzeigen", reloadAchievements: "Erneut lesen", loading: "Lese …",
    earned: "freigeschaltet · direkt vom Steam-Client gelesen", noAchievements: "Steam meldet für dieses Spiel keine Errungenschaften.",
    steamRequestNote: "Steam verarbeitet die Anfrage für dein angemeldetes Konto. Experimentelle Client-Schnittstelle · Nur lesend",
    prototype: "Experimenteller lokaler Prototyp", windowsSteam: "GameVault · Windows · Steam",
    detectedNote: "Lokal installierte Spiele sind nur Kandidaten und bilden nicht die vollständige Bibliothek ab. Steam wird nur bei einer ausdrücklichen Anfrage zu Errungenschaften kontaktiert.",
    noSteam: "Steam wurde an den üblichen Orten nicht gefunden.",
    workerCrashed: "Die isolierte Steam-Lesehilfe ist abgestürzt (Code {code}); GameVault blieb davon getrennt. Letzter protokollierter Schritt: {stage}.",
    errSteamLibrary: "Steam-Client-Bibliothek konnte nicht geladen werden.", errInterfaces: "Die erwarteten Steam-Lese-Schnittstellen sind nicht verfügbar.",
    errStatsInterface: "Steam stellt die angeforderte Schnittstelle SteamUserStats013 nicht bereit.", errUserInterface: "Steam stellt die angeforderte Schnittstelle SteamUser012 nicht bereit.",
    errSession: "Keine angemeldete Steam-Sitzung gefunden.", errTimeout: "Steam hat die Errungenschaftsanfrage nicht rechtzeitig beantwortet.",
    errNotInstalled: "Das Spiel ist nicht in einer lokal geprüften Steam-Bibliothek installiert.", errInvalid: "Die Steam-Anfrage oder App-ID ist ungültig.",
    errReaderStart: "Die isolierte Steam-Lesehilfe konnte nicht gestartet werden.", errReaderReply: "Die isolierte Steam-Lesehilfe lieferte keine gültige oder vollständige Antwort.",
    errDefault: "Errungenschaften konnten nicht gelesen werden.",
    language: "Sprache", achievedDate: "Freigeschaltet am {date}",
  },
  fr: {
    tagline: "VOTRE PROGRESSION DE JEU. AU MÊME ENDROIT.", localOnly: "LOCAL UNIQUEMENT", steamLibrary: "BIBLIOTHÈQUE STEAM",
    headline: "Tous vos jeux, en un coup d’œil.", intro: "GameVault recherche les installations Steam locales et affiche les données disponibles sur ce PC.",
    scan: "Analyser Steam localement", scanning: "Analyse…", overview: "Aperçu", localReadOnly: "Détection locale · Lecture seule",
    gamesFound: "{count} jeux locaux", notScanned: "Pas encore analysé", waitingTitle: "Votre bibliothèque vous attend",
    waitingText: "Lancez une analyse locale. GameVault ne modifie pas les fichiers Steam et n’a pas besoin de vos identifiants.",
    steamInstall: "INSTALLATION STEAM", found: "Trouvée", notFound: "Introuvable", commonLocations: "Emplacements d’installation habituels vérifiés",
    steamSession: "SESSION STEAM", notConnected: "Non connecté", sessionDetail: "Connexion uniquement lors d’une lecture demandée",
    achievements: "SUCCÈS", dataRead: "Données lues", unavailable: "Indisponibles", notRead: "Pas encore lus",
    achievementsDetail: "Consultables sur demande pour chaque jeu installé", incompleteScan: "Certaines entrées locales n’ont pas pu être vérifiées en raison de restrictions d’accès ou de limites de sécurité.",
    inspectError: "Impossible de vérifier les dossiers Steam.", libraryTitle: "Bibliothèque locale", installed: "installé localement",
    loadAchievements: "Afficher les succès", reloadAchievements: "Relire", loading: "Lecture…",
    earned: "débloqués · lus directement depuis le client Steam", noAchievements: "Steam ne signale aucun succès pour ce jeu.",
    steamRequestNote: "Steam traite cette demande pour le compte connecté. Interface client expérimentale · Lecture seule",
    prototype: "Prototype local expérimental", windowsSteam: "GameVault · Windows · Steam",
    detectedNote: "Les jeux installés détectés ne représentent pas la bibliothèque complète du compte. Steam n’est contacté que sur demande explicite pour lire les succès.",
    noSteam: "Steam est introuvable dans les emplacements habituels.",
    workerCrashed: "Le lecteur Steam isolé a planté (code {code}) ; GameVault est resté séparé. Dernière étape enregistrée : {stage}.",
    errSteamLibrary: "Impossible de charger la bibliothèque du client Steam.", errInterfaces: "Les interfaces de lecture Steam attendues sont indisponibles.",
    errStatsInterface: "Steam ne fournit pas l’interface SteamUserStats013 demandée.", errUserInterface: "Steam ne fournit pas l’interface SteamUser012 demandée.",
    errSession: "Aucune session Steam connectée n’a été trouvée.", errTimeout: "Steam n’a pas répondu à temps à la demande de succès.",
    errNotInstalled: "Ce jeu n’est pas installé dans une bibliothèque Steam locale vérifiée.", errInvalid: "La demande Steam ou l’identifiant de l’application est invalide.",
    errReaderStart: "Impossible de démarrer le lecteur Steam isolé.", errReaderReply: "Le lecteur Steam isolé a renvoyé une réponse invalide ou incomplète.",
    errDefault: "Impossible de lire les données de succès.",
    language: "Langue", achievedDate: "Débloqué le {date}",
  },
  es: {
    tagline: "TU PROGRESO EN LOS JUEGOS. EN UN SOLO LUGAR.", localOnly: "SOLO LOCAL", steamLibrary: "BIBLIOTECA DE STEAM",
    headline: "Tus juegos, de un vistazo.", intro: "GameVault busca instalaciones locales de Steam y muestra los datos disponibles en este PC.",
    scan: "Buscar Steam localmente", scanning: "Buscando…", overview: "Resumen", localReadOnly: "Detección local · Solo lectura",
    gamesFound: "{count} juegos locales", notScanned: "Aún no se ha buscado", waitingTitle: "Tu biblioteca te espera",
    waitingText: "Inicia una búsqueda local. GameVault no modifica archivos de Steam ni necesita tus credenciales.",
    steamInstall: "INSTALACIÓN DE STEAM", found: "Encontrada", notFound: "No encontrada", commonLocations: "Se comprobaron las ubicaciones habituales",
    steamSession: "SESIÓN DE STEAM", notConnected: "Sin conexión", sessionDetail: "Solo se conecta para una lectura solicitada",
    achievements: "LOGROS", dataRead: "Datos leídos", unavailable: "No disponibles", notRead: "Sin leer",
    achievementsDetail: "Disponibles bajo petición para cada juego instalado", incompleteScan: "No se pudieron comprobar algunas entradas locales por restricciones de acceso o límites de seguridad.",
    inspectError: "No se pudieron comprobar las carpetas de Steam.", libraryTitle: "Biblioteca local", installed: "instalado localmente",
    loadAchievements: "Mostrar logros", reloadAchievements: "Volver a leer", loading: "Leyendo…",
    earned: "desbloqueados · leídos directamente del cliente de Steam", noAchievements: "Steam no indica logros para este juego.",
    steamRequestNote: "Steam procesa esta solicitud para tu cuenta con sesión iniciada. Interfaz experimental · Solo lectura",
    prototype: "Prototipo local experimental", windowsSteam: "GameVault · Windows · Steam",
    detectedNote: "Los juegos instalados detectados son solo una parte; no representan toda la biblioteca de la cuenta. Steam solo se contacta si solicitas leer logros.",
    noSteam: "No se encontró Steam en las ubicaciones habituales.",
    workerCrashed: "El lector aislado de Steam se cerró con error (código {code}); GameVault permaneció separado. Último paso registrado: {stage}.",
    errSteamLibrary: "No se pudo cargar la biblioteca del cliente de Steam.", errInterfaces: "Las interfaces de lectura de Steam esperadas no están disponibles.",
    errStatsInterface: "Steam no ofrece la interfaz solicitada SteamUserStats013.", errUserInterface: "Steam no ofrece la interfaz solicitada SteamUser012.",
    errSession: "No se encontró una sesión de Steam iniciada.", errTimeout: "Steam no respondió a tiempo a la solicitud de logros.",
    errNotInstalled: "Este juego no está instalado en una biblioteca local de Steam comprobada.", errInvalid: "La solicitud de Steam o el ID de la aplicación no es válido.",
    errReaderStart: "No se pudo iniciar el lector aislado de Steam.", errReaderReply: "El lector aislado de Steam devolvió una respuesta no válida o incompleta.",
    errDefault: "No se pudieron leer los datos de logros.",
    language: "Idioma", achievedDate: "Desbloqueado el {date}",
  },
  pt: {
    tagline: "O SEU PROGRESSO NOS JOGOS. NUM SÓ LUGAR.", localOnly: "APENAS LOCAL", steamLibrary: "BIBLIOTECA STEAM",
    headline: "Os seus jogos, num relance.", intro: "O GameVault verifica instalações locais do Steam e mostra os dados disponíveis neste PC.",
    scan: "Verificar o Steam localmente", scanning: "A verificar…", overview: "Visão geral", localReadOnly: "Deteção local · Apenas leitura",
    gamesFound: "{count} jogos locais", notScanned: "Ainda não verificado", waitingTitle: "A sua biblioteca está à espera",
    waitingText: "Inicie uma verificação local. O GameVault não altera ficheiros do Steam nem precisa das suas credenciais.",
    steamInstall: "INSTALAÇÃO DO STEAM", found: "Encontrada", notFound: "Não encontrada", commonLocations: "Foram verificados os locais de instalação habituais",
    steamSession: "SESSÃO STEAM", notConnected: "Não ligado", sessionDetail: "Liga-se apenas durante uma leitura pedida",
    achievements: "CONQUISTAS", dataRead: "Dados lidos", unavailable: "Indisponíveis", notRead: "Não lidas",
    achievementsDetail: "Disponíveis mediante pedido para cada jogo instalado", incompleteScan: "Não foi possível verificar algumas entradas locais devido a restrições de acesso ou limites de segurança.",
    inspectError: "Não foi possível verificar as pastas do Steam.", libraryTitle: "Biblioteca local", installed: "instalado localmente",
    loadAchievements: "Mostrar conquistas", reloadAchievements: "Ler novamente", loading: "A ler…",
    earned: "desbloqueadas · lidas diretamente do cliente Steam", noAchievements: "O Steam não indica conquistas para este jogo.",
    steamRequestNote: "O Steam processa este pedido para a sua conta com sessão iniciada. Interface experimental · Apenas leitura",
    prototype: "Protótipo local experimental", windowsSteam: "GameVault · Windows · Steam",
    detectedNote: "Os jogos instalados detetados são apenas uma parte e não representam a biblioteca completa da conta. O Steam só é contactado quando pede a leitura de conquistas.",
    noSteam: "O Steam não foi encontrado nos locais habituais.",
    workerCrashed: "O leitor isolado do Steam falhou (código {code}); o GameVault manteve-se separado. Última etapa registada: {stage}.",
    errSteamLibrary: "Não foi possível carregar a biblioteca do cliente Steam.", errInterfaces: "As interfaces de leitura Steam esperadas não estão disponíveis.",
    errStatsInterface: "O Steam não disponibiliza a interface pedida SteamUserStats013.", errUserInterface: "O Steam não disponibiliza a interface pedida SteamUser012.",
    errSession: "Não foi encontrada uma sessão Steam iniciada.", errTimeout: "O Steam não respondeu a tempo ao pedido de conquistas.",
    errNotInstalled: "Este jogo não está instalado numa biblioteca Steam local verificada.", errInvalid: "O pedido Steam ou o ID da aplicação é inválido.",
    errReaderStart: "Não foi possível iniciar o leitor Steam isolado.", errReaderReply: "O leitor Steam isolado devolveu uma resposta inválida ou incompleta.",
    errDefault: "Não foi possível ler os dados das conquistas.",
    language: "Idioma", achievedDate: "Desbloqueada em {date}",
  },
} as const;

export type TranslationKey = keyof typeof messages.en;

export function translate(language: Language, key: TranslationKey, values: Record<string, string | number> = {}): string {
  let message: string = messages[language][key];
  for (const [name, value] of Object.entries(values)) message = message.split(`{${name}}`).join(String(value));
  return message;
}

export function detectLanguage(): Language {
  const saved = localStorage.getItem("gamevault-language");
  if (languages.includes(saved as Language)) return saved as Language;
  const browserLanguage = navigator.language.toLowerCase().split("-")[0];
  return languages.includes(browserLanguage as Language) ? browserLanguage as Language : "en";
}

export function localizeSteamError(message: string | null, language: Language): string {
  if (!message) return translate(language, "errDefault");
  const value = message.toLocaleLowerCase();
  const crash = message.match(/\(Code ([^)]+)\).*?Letzter protokollierter Schritt: ([^.]+)\./i);
  if (crash) return translate(language, "workerCrashed", { code: crash[1], stage: crash[2] });
  const key: TranslationKey = value.includes("steamuserstats013") ? "errStatsInterface"
    : value.includes("steamuser012") ? "errUserInterface"
      : value.includes("schnittstellen") || value.includes("interfaces") ? "errInterfaces"
        : value.includes("bibliothek konnte nicht geladen") || value.includes("library could not be loaded") ? "errSteamLibrary"
          : value.includes("angemeldete steam-sitzung") || value.includes("signed-in steam session") ? "errSession"
            : value.includes("nicht rechtzeitig") || value.includes("in time") || value.includes("timeout") ? "errTimeout"
              : value.includes("nicht in einer lokal geprüften") || value.includes("not installed in a locally checked") ? "errNotInstalled"
                : value.includes("app-id") || value.includes("app id") || value.includes("ungültige steam-leseanfrage") ? "errInvalid"
                  : value.includes("lesehilfe") && value.includes("start") ? "errReaderStart"
                    : value.includes("ungültige antwort") || value.includes("invalid response") ? "errReaderReply"
                      : "errDefault";
  return translate(language, key);
}
