Map<String, dynamic> initialData = Map<String, dynamic>();

this.initialData = const {
        'Header': 'Match Scouting',
        'Team Number': 0,
        'Match Number': 0,
'yes':'',
'cool':'',

//new question

'nice':'',
'very':'',

//new question

'Question':['answer2','answer1',],
'Question2':['answer2','answer1',],
};
List<Question>? matchFormQuestions;
matchFormQuestions = [
ShortAnswer(
'cool',
TextInputType.number,
initialValue: widget.initialData['cool'],
),
ShortAnswer(
'yes',
TextInputType.number,
initialValue: widget.initialData['yes'],
),
ShortAnswer(
'very',
TextInputType.number,
initialValue: widget.initialData['very'],
),
ShortAnswer(
'nice',
TextInputType.number,
initialValue: widget.initialData['nice'],
),
DropDownQuestion(Question,
['answer2','answer1',],
,answer: widget.initialData[Question],),
DropDownQuestion(Question2,
['answer2','answer1',],
,answer: widget.initialData[Question2],),
];
