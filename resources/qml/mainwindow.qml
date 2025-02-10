import QtQuick
import QtQuick.Dialogs
import QtQuick.Controls
import QtQuick.Layouts

import AOCSolutions

ApplicationWindow {
    visible: true
    minimumWidth: 400
    minimumHeight: 250
    width: 800
    height: 600

    AOCSolutions {
        id: solutions
        inputs_dir: inputsPath.text
    }
    
    FolderDialog {
        id: folderPicker
        
        onAccepted: {
            inputsPath.text = folderPicker.currentFolder.toString().slice(7)
        }
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        RowLayout {
            id: topRow
            Layout.alignment: Qt.AlignTop
            Layout.margins: 20

            TextField {
                id: inputsPath
                Layout.fillWidth: true
            }

            Button {
                text: "Inputs directory"

                onClicked: {
                    folderPicker.visible = true
                }
            }
        }

        GridLayout {
            Layout.alignment: Qt.AlignHCenter
            Layout.margins: 20

            columns: 5
            rows:5
            
            Button {
                text: "Day 1"

                onClicked: {
                    solutions.request_solution(1)
                }
            }
            
            Button {
                text: "Day 2"
                
                onClicked: {
                    solutions.request_solution(2)
                }
            }
            
            Button {
                text: "Day 3"
                
                onClicked: {
                    solutions.request_solution(3)
                }
            }
        }

        TextArea {
            id: display
            Layout.alignment: Qt.AlignHCenter
            Layout.margins: 20
            Layout.fillWidth: true
            Layout.fillHeight: true
                
            readOnly: true
            font.family: "Noto Sans Mono"
            horizontalAlignment: TextEdit.AlignHCenter
            text: solutions.output
        }
    }
}